use capnp;
use zmq;
use capnp_zmq;
use std;
use time;
use explorers_capnp::Grid;

enum OutputMode {
    Colors,
    Confidence
}

fn write_ppm(path : &std::path::Path, grid : Grid::Reader, mode : OutputMode) -> std::io::IoResult<()> {
    match std::io::File::open_mode(path, std::io::Truncate, std::io::Write) {
        Err(_e) => fail!("could not open"),
        Ok(writer) => {
            let mut buffered = std::io::BufferedWriter::new(writer);
            writeln!(&mut buffered, "P6");

            let cells = grid.get_cells();
            let width = cells.size();
            assert!(width > 0);
            let height = cells[0].size();

            writeln!(&mut buffered, "{} {}", width, height);
            writeln!(&mut buffered, "255");

            for x in range(0, width) {
                assert!(cells[x].size() == height);
            }

            for y in range(0, height) {
                for x in range(0, width) {
                    let cell = cells[x][y];

                    match mode {
                        Colors => {
                            try!(buffered.write_u8((cell.get_mean_red()).floor() as u8));
                            try!(buffered.write_u8((cell.get_mean_green()).floor() as u8));
                            try!(buffered.write_u8((cell.get_mean_blue()).floor() as u8));
                        }
                        Confidence => {
                            let mut age = time::now().to_timespec().sec - cell.get_latest_timestamp();
                            if age < 0 { age = 0 };
                            age *= 25;
                            if age > 255 { age = 255 };
                            age = 255 - age;

                            let mut n = cell.get_number_of_updates();
                            n *= 10;
                            if n > 255 { n = 255 };

                            try!(buffered.write_u8(0 as u8));

                            try!(buffered.write_u8(n as u8));

                            try!(buffered.write_u8(age as u8));
                        }
                    }
                }
            }
            try!(buffered.flush());
        }
    }
    Ok(())
}

pub fn main() -> Result<(), zmq::Error> {
    use capnp::message::MessageReader;

    let mut context = zmq::Context::new();
    let mut requester = try!(context.socket(zmq::REQ));

    try!(requester.connect("tcp://localhost:5556"));

    let mut c : uint = 0;

    loop {
        try!(requester.send([], 0));

        let frames = try!(capnp_zmq::recv(&mut requester));
        let segments = capnp_zmq::frames_to_segments(frames);
        let reader = capnp::message::SegmentArrayMessageReader::new(segments,
                                                                    capnp::message::DefaultReaderOptions);
        let grid = reader.get_root::<Grid::Reader>();

        println!("{}", grid.get_latest_timestamp());

        let filename = std::path::Path::new(format!("colors{:05}.ppm", c));
        write_ppm(&filename, grid, Colors).unwrap();

        let filename = std::path::Path::new(format!("conf{:05}.ppm", c));
        write_ppm(&filename, grid, Confidence).unwrap();

        c += 1;
        std::io::timer::sleep(5000);
    }
}
