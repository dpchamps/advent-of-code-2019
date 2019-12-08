use adventofcode::read_input_file;

type Layer = Vec<i32>;
type Image = Vec<Layer>;

fn read_image_stream(height : i32, width : i32, image_stream : &[i32]) -> Image {
    let mut cols = 0;
    let mut image: Image = Vec::new();
    let mut current_layer: Layer = Vec::new();

    for byte in image_stream{
        if cols > width*height-1 {
            cols = 0;
            image.push(current_layer);
            current_layer = Vec::new();
        }

        current_layer.push(*byte);
        cols += 1;
    }

    image.push(current_layer);

    image
}

fn count_digits_in_layer(layer : &Layer, digit : i32) -> i32 {
    layer.iter().fold(0, |count, item| {
        if *item == digit{
            return count + 1
        }

        count
    })
}

fn layer_with_fewest_digits(image : &Image, digit : i32) -> &Layer {
    let mut final_layer = &image[0];
    let mut fewest = count_digits_in_layer(final_layer, digit);

    for layer in image{
        let next = count_digits_in_layer(layer, digit);

        if(next < fewest){
            final_layer = layer;
            fewest = next;
        }
    }

    final_layer
}

fn collapse_layers(image : &Image) -> Layer{
    let mut resolved_layer = image[0].clone();

    for layer in image[1..].iter(){
        for index in 0..resolved_layer.len(){
            let pixel = resolved_layer[index];
            if pixel == 2{
                resolved_layer[index] = layer[index];
            }
        }
    }

    resolved_layer
}

fn display_layer(layer : &Layer, width : i32){
    println!("Printing {:?}", layer.len());
    let mut col = 0;
    let mut line : Vec<String> = Vec::new();
    for pixel in layer{
        if col > width-1{
            println!("{}", line.join(""));
            col = 0;
            line = Vec::new()
        }

        let output = match pixel{
            0 => "■",
            1 => "□",
            _ => "?"
        };

        line.push(output.to_string());
        col += 1;
    }

    println!("{}", line.join(""));
}

fn part_one(){
    let file = read_input_file("day-8-part-1-input");
    let image_stream: Vec<i32> = file.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    let image = read_image_stream(6, 25, &image_stream);

    let fewest_zeros = layer_with_fewest_digits(&image, 0);
    let ones_times_twos = count_digits_in_layer(fewest_zeros, 1) * count_digits_in_layer(fewest_zeros, 2);

    println!("Part 1: {}", ones_times_twos);

}

fn part_two(){
    let file = read_input_file("day-8-part-1-input");
    let image_stream: Vec<i32> = file.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    let image = read_image_stream(6, 25, &image_stream);

    let collapsed = collapse_layers(&image);
    display_layer(&collapsed, 25);
}

fn main(){
    part_one();
    part_two();
}


#[cfg(test)]
mod day_8_tests{
    use crate::{read_image_stream, count_digits_in_layer, layer_with_fewest_digits};

    #[test]
    fn sanity_check(){
        let stream = vec![1,2,3,4,5,6,7,8,9,0,1,2];
        let image = read_image_stream(2, 3, &stream);

        assert_eq!(
            image,
            vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
        )
    }

    #[test]
    fn count_items_in_layer(){
        let layer = vec![1,1,1,0,0,0,0,2,2,2];

        assert_eq!(
            count_digits_in_layer(&layer, 1),
            3
        );
        assert_eq!(
            count_digits_in_layer(&layer, 0),
            4
        );
        assert_eq!(
            count_digits_in_layer(&layer, 2),
            3
        );
    }

    #[test]
    fn find_fewest_in_layer(){
        let image = vec![
            vec![0,0,0,0,0,1,1,1,1,1],
            vec![0,0,0,0,0,0,1,1,1,1],
            vec![0,0,0,2,2,2,1,2,3,3],
        ];

        let fewest_zeros = layer_with_fewest_digits(&image, 0);

        assert_eq!(
            fewest_zeros.clone(),
            vec![0,0,0,2,2,2,1,2,3,3]
        )
    }
}