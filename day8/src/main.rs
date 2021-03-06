use std::fs::read_to_string;

fn main() {
    let image_string = read_to_string("input.txt").unwrap();
    let image = Image::init(&image_string[..image_string.len() - 1], ImageCoordinate{x: 25, y: 6});
    //part1(image);
    part2(image);
}

#[allow(dead_code)]
fn part1(image: Image) {
    let layer_id = image.get_layer_id_with_lowest_digit_count(' ');
    println!("Layer with lowest ' ' count: {}", layer_id);
    let nr_digits_1 = image.count_digits_on_layer(layer_id, '1');
    let nr_digits_2 = image.count_digits_on_layer(layer_id, '2');
    println!("It has {} '1' and {} '2'. {} * {} = {}", nr_digits_1, nr_digits_2, nr_digits_1, nr_digits_2, nr_digits_1 * nr_digits_2);
}

fn part2(image: Image) {
    println!("{}", image.render());
}

struct ImageCoordinate {
    x: usize,
    y: usize,
}

struct Image {
    data: String,
    size: ImageCoordinate,
    nr_layers: usize,
    digits_per_layer: usize,
}

impl Image {
    fn init(data: &str, size: ImageCoordinate) -> Image {
        let digits_per_layer = size.x * size.y;
        assert_eq!(data.len() % digits_per_layer, 0);
        let nr_layers = data.len() / digits_per_layer;
        assert_eq!(data.len(), digits_per_layer * nr_layers);
        return Image {data: String::from(data), size, nr_layers, digits_per_layer};
    }

    fn render(&self) -> String {
        let mut render_string = String::from("");
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                render_string.push(self.get_pixel(ImageCoordinate{x, y}));
            }
            if y != self.size.y - 1 {
                render_string.push('\n');
            }
        }
        return render_string;
    }

    fn image_coordinate_to_str_coordinate(&self, image_coordinate: &ImageCoordinate) -> usize {
        assert_eq!(image_coordinate.x < self.size.x, true);
        assert_eq!(image_coordinate.y < self.size.y, true);
        let str_coordinate = image_coordinate.x + self.size.x * image_coordinate.y;
        assert_eq!(str_coordinate < self.digits_per_layer, true);
        return str_coordinate;
    }

    fn get_pixel(&self, at: ImageCoordinate) -> char {
        for layer_id in 0..self.nr_layers {
            let current_layer = self.layer(layer_id);
            let layer_str_coordinate = self.image_coordinate_to_str_coordinate(&at);
            match &current_layer[layer_str_coordinate..layer_str_coordinate+1] {
                "0" => return ' ',
                "1" => return '1',
                _ => continue,
            }
        }
        panic!("All pixels seem transparent!");
    }

    fn layer(&self, layer_id: usize) -> &str {
        return &self.data[layer_id * self.digits_per_layer..(layer_id + 1) * self.digits_per_layer];
    }

    fn count_digits_on_layer(&self, layer_id: usize, digit: char) -> usize {
        let mut count = 0;
        for layer_digit in self.layer(layer_id).chars() {
            if layer_digit == digit {
                count += 1;
            }
        }
        return count;
    }

    fn get_layer_id_with_lowest_digit_count(&self, digit: char) -> usize {
        let mut lowest_count = std::usize::MAX;
        let mut lowest_count_layer_id = 0;
        for layer_id in 0..self.nr_layers {
            let digit_count = self.count_digits_on_layer(layer_id, digit);
            if digit_count < lowest_count {
                lowest_count = digit_count;
                lowest_count_layer_id = layer_id;
            }
        }
        return lowest_count_layer_id;
    }
}

#[test]
#[should_panic]
fn image_init_panics_when_data_is_lost() {
    let image_string = "1234567890121";
    Image::init(&image_string, ImageCoordinate{x: 3, y: 2});
}

#[test]
#[should_panic]
fn image_init_panics_when_too_big_size() {
    let image_string = "123456789012";
    Image::init(&image_string, ImageCoordinate{x: 3, y: 3});
}

#[test]
#[should_panic]
fn image_coordinate_conversion_panics_y() {
    let image_string = "123456789012";
    let image_size = ImageCoordinate{x: 3, y: 2};
    let image = Image::init(&image_string, image_size);
    image.image_coordinate_to_str_coordinate(&ImageCoordinate{x: 2, y: 2});
}

#[test]
#[should_panic]
fn image_coordinate_conversion_panics_x() {
    let image_string = "123456789012";
    let image_size = ImageCoordinate{x: 3, y: 2};
    let image = Image::init(&image_string, image_size);
    image.image_coordinate_to_str_coordinate(&ImageCoordinate{x: 3, y: 1});
}

#[test]
fn image_coordinate_conversion() {
    let image_string = "123456789012";
    let image_size = ImageCoordinate{x: 3, y: 2};
    let image = Image::init(&image_string, image_size);
    assert_eq!(image.image_coordinate_to_str_coordinate(&ImageCoordinate{x: 2, y: 1}), 5);
    assert_eq!(image.image_coordinate_to_str_coordinate(&ImageCoordinate{x: 0, y: 0}), 0);
    assert_eq!(image.image_coordinate_to_str_coordinate(&ImageCoordinate{x: 2, y: 0}), 2);
}

#[test]
fn image_layer_access() {
    let image_string = "123456789012";
    let image_size = ImageCoordinate{x: 3, y: 2};
    let image = Image::init(&image_string, image_size);
    assert_eq!(image.nr_layers, 2);
    assert_eq!(image.layer(0), "123456");
    assert_eq!(image.layer(1), "789012");
}

#[test]
fn digit_count() {
    let image_string = "123456789012";
    let image_size = ImageCoordinate{x: 3, y: 2};
    let image = Image::init(&image_string, image_size);
    assert_eq!(image.count_digits_on_layer(0, 'd'), 0);
    assert_eq!(image.count_digits_on_layer(0, '1'), 1);
    assert_eq!(image.count_digits_on_layer(0, '9'), 0);
    assert_eq!(image.count_digits_on_layer(1, '1'), 1);
    assert_eq!(image.count_digits_on_layer(1, '9'), 1);
    assert_eq!(image.count_digits_on_layer(1, '5'), 0);
}

#[test]
fn get_pixel_of_test_image() {
    let image_string = "0222112222120000";
    let image_size = ImageCoordinate{x: 2, y: 2};
    let image = Image::init(&image_string, image_size);
    assert_eq!(image.get_pixel(ImageCoordinate{x: 0, y: 0}), ' ');
    assert_eq!(image.get_pixel(ImageCoordinate{x: 1, y: 0}), '1');
    assert_eq!(image.get_pixel(ImageCoordinate{x: 0, y: 1}), '1');
    assert_eq!(image.get_pixel(ImageCoordinate{x: 1, y: 1}), ' ');
}

#[test]
fn visualize_test_image() {
    let image_string = "0222112222120000";
    let image_size = ImageCoordinate{x: 2, y: 2};
    let image = Image::init(&image_string, image_size);
    let image_visualization = image.render();
    assert_eq!(image_visualization, " 1\n1 ");
}
