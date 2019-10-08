use std::fs::File;
use std::io::Write;

pub fn write_to_file(file_location: &str,
                        weights_input_one: &Vec<Vec<f32>>, weights_one_two: &Vec<Vec<f32>>, weights_two_output: &Vec<Vec<f32>>,
                        bias_one: &Vec<f32>, bias_two: &Vec<f32>, bias_output: &Vec<f32>
        ) -> Result<File, std::io::Error> {

    //open the file
    let mut file = match File::create(file_location) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };

    //weights
        //input one
        for input in weights_input_one {
            file = write_line(file, input)?;
        }
        write!(file, "w\n")?;

        //one two
        for input in weights_one_two {
            file = write_line(file, input)?;
        }
        write!(file, "w\n")?;

        //two output
        for input in weights_two_output {
            file = write_line(file, input)?;
        }

    write!(file, "#\n")?;

    //biases
        //one
        file = write_line(file, bias_one)?;
        
        //two
        file = write_line(file, bias_two)?;

        //output
        file = write_line(file, bias_output)?;

    Ok(file)
}

pub fn read_weights_biases(weights_biases_string: String) -> Result<(usize, usize, Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<f32>, Vec<f32>, Vec<f32>, usize), String>{
    //split into weights and biases
    let weights_biases:Vec<&str> = weights_biases_string.split("\n#\n").collect();
    if weights_biases.len() != 2 {
        return Err(String::from("Weights string contains incorrect number of # symbols, there should be one on its own line"));
    }

    //get the weights
        let weights:Vec<&str> = weights_biases[0].split("\nw\n").collect();
        if weights.len() != 3 {
            return Err(String::from("Incorrect number of weights, there should be 3, separated by 2 ws each on their own lines"));
        }

        //input
        //split into source nodes
        let weights_input:Vec<&str> = weights[0].split('\n').collect();
        let num_inputs = weights_input.len();
        //split into weights
        let weights_input_one:Vec<Vec<f32>> = double_split_parse(weights_input);
        let num_layer_one = weights_input_one[0].len();
        
        //layer one
        //split into source nodes
        let weights_one:Vec<&str> = weights[1].split('\n').collect();
        if weights_one.len() != num_layer_one {
            return Err(String::from("Different number of nodes in layer 1"))
        }
        //split into weights
        let weights_one_two:Vec<Vec<f32>> = double_split_parse(weights_one);
        let num_layer_two = weights_one_two[0].len();

        //layer two
        //split into source nodes
        let weights_two:Vec<&str> = weights[2].split('\n').collect();
        if weights_two.len() != num_layer_two {
            return Err(String::from("Different number of nodes in layer 2"))
        }
        //split into weights
        let weights_two_output:Vec<Vec<f32>> = double_split_parse(weights_two);


    //the biases
        let bias_layers:Vec<&str> = weights_biases[1].split('\n').collect();

        //layer one
        let bias_one:Vec<f32> = split_parse(bias_layers[0]);
        if bias_one.len() != num_layer_one {
            return Err(String::from("Wrong number of biases for layer 1"));
        }

        //layer two
        let bias_two:Vec<f32> = split_parse(bias_layers[1]);
        if bias_two.len() != num_layer_two {
            return Err(String::from("Wrong number of biases for layer 2"));
        }

        //output
        let bias_output:Vec<f32> = split_parse(bias_layers[2]);
        if bias_output.len() != 1 {
            return Err(String::from("Wrong number of output biases"));
        }
        
    Ok((num_layer_one, num_layer_two,
    weights_input_one, weights_one_two, weights_two_output,
    bias_one, bias_two, bias_output,
    num_inputs))
}

//takes a string and parses it into a vector of t's
fn split_parse<T>(input: &str) -> Vec<T> 
    where T: std::str::FromStr,
          T::Err : std::fmt::Debug
{
    //split it, parse each value and unwrap it, then collect into a vector
    input.split(' ').map(|x| x.parse::<T>().unwrap()).collect()
}

//takes a vector of strings and parses it into a vector of vectors of t's
fn double_split_parse<T>(input: Vec<&str>) -> Vec<Vec<T>> 
    where T: std::str::FromStr,
          T::Err : std::fmt::Debug
{
    //turn the vector into an iterator, parse each string, then collect them back into a vector
    input.iter().map(|x| split_parse(x)).collect()
}

//takes a file and a vector, prints them onto a line and returns a result of the file
fn write_line(mut file: std::fs::File, list: &Vec<f32>) -> Result<File, std::io::Error> {
    //but don't print a space after the last value
    let len: usize = list.len() - 1;
    for num in 0..len {
        //the question mark detects and returns any errors, ensuring our code is always valid
        write!(file, "{:.6} ", list[num])?;
    }
    write!(file, "{:.6}", list[len])?;
    write!(file, "\n")?;
    //return the file
    return Ok(file);
}