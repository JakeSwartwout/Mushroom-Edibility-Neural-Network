//a nerual network to take in information about a mushroom and determine whether it is edible or poisonous


//have user input a mushroom
//read/write weights to a file

use std::fs;

mod mushrooms;
use mushrooms::*;

mod vector_manipulation;
use vector_manipulation::*;

mod input_output;
use input_output::*;

fn main() {
    //read the string and make sure it's valid
    match fs::read_to_string("agaricus-lepiota.data"){
        Err(e) => {
            println!("Cannot open file: {:?}", e);
        }
        Ok(contents) => { //can read the file

            //get the training data
                //each element in mushrooms is an f32 vector representing the attributes for that mushroom
                //each element in mushroom_results is an f32 representing whether it is edible or not
                let (mut mushrooms, mut mushroom_results) = get_mushrooms(contents.trim());

                //make sure there are at least 10 mushrooms
                if let None = mushrooms.get(10){
                    panic!("There aren't enough mushrooms");
                }

            //split into training and testing
                //each item in samples is a vector of 1.0s and 0.0 representing the attributes
                //each item in results is either 1.0 or 0.0 representing edible or poisonous
                let percent_train = 0.75;
                let size_training = (percent_train * (mushrooms.len() as f32)) as usize;

                let testing_samples = mushrooms.split_off(size_training);
                let testing_results = mushroom_results.split_off(size_training);

                let training_samples = mushrooms;
                let training_results = mushroom_results;

                let size_testing = testing_samples.len();

            //create the neurons and either read or randomize the weights
                //remember, each mushroom is a 1d vector
                //the output should be a single neuron

                let num_inputs: usize = training_samples.get(0).unwrap().len();
                let num_layer_one: usize;
                let num_layer_two: usize;

                //to catch the outputs
                let mut layer_one: Vec<f32>;
                let mut layer_two: Vec<f32>;
                let mut output_layer: Vec<f32>;

                //create the weights
                //the outer vector correlates to the input neuron, the inner one to the outgoing weight
                //it is a list of neurons, which contains a list of the outgoing weights
                let mut weights_input_one: Vec<Vec<f32>>;
                let mut weights_one_two: Vec<Vec<f32>>;
                let mut weights_two_output: Vec<Vec<f32>>;

                //create the biases
                //bias is added to the node after summing up the weighted inputs (I think, it's what makes sense to me)
                let mut bias_one: Vec<f32>;
                let mut bias_two: Vec<f32>;
                let mut bias_output: Vec<f32>;

                //check if the weights already exist
                match fs::read_to_string("mushroom_weights.txt"){
                    Err(_e) => {
                        //create them the right size and randomize

                        num_layer_one = 25;
                        num_layer_two = 10;

                        //the outer vector correlates to the input neuron, the inner one to the outgoing weight
                        weights_input_one = vec![vec![0.0; num_layer_one]; num_inputs];
                        weights_one_two = vec![vec![0.0; num_layer_two]; num_layer_one];
                        weights_two_output = vec![vec![0.0; 1]; num_layer_two];

                        //create the biases
                        //bias is added to the node after summing up the weighted inputs (I think, it's what makes sense to me)
                        bias_one = vec![0.0; num_layer_one];
                        bias_two = vec![0.0; num_layer_two];
                        bias_output = vec![0.0; 1];

                        //randomize them
                        weights_input_one.fill_with_random(-1.0, 1.0);
                        weights_one_two.fill_with_random(-1.0, 1.0);
                        weights_two_output.fill_with_random(-1.0, 1.0);

                        bias_one.fill_with_random(-1.0, 0.0);
                        bias_two.fill_with_random(-1.0, 0.0);
                        bias_output.fill_with_random(-1.0, 0.0);
                    },
                    Ok(weights_biases_string) => {
                        match read_weights_biases(weights_biases_string) {
                            Err(e) => panic!("Error reading weights/biases: {}", e),
                            Ok(tuple) => {
                                num_layer_one = tuple.0;
                                num_layer_two = tuple.1;
                                weights_input_one = tuple.2;
                                weights_one_two = tuple.3;
                                weights_two_output = tuple.4;
                                bias_one = tuple.5;
                                bias_two = tuple.6;
                                bias_output = tuple.7;
                                if num_inputs != tuple.8 {
                                    panic!("Number of read inputs does not match actual inputs");
                                }
                            }
                        }
                    }
                }

            //train on each mushroom, doing 3 loops
            'outer_loop: for pass in 0..2 {
                println!("Starting pass {}", pass + 1);
                for curr_train in 0..size_training {

                    //pass through
                        match pass_through( &training_samples[curr_train],
                                            &weights_input_one, &bias_one,
                                            &weights_one_two, &bias_two,
                                            &weights_two_output, &bias_output)
                        {
                            Err(e) => {   
                                println!("Something went wrong on pass {} for mushroom {}", pass, curr_train);
                                println!("Error: {}", e);
                                break 'outer_loop;
                            }
                            Ok((layer_one_new, layer_two_new, output_layer_new)) =>{
                                layer_one = layer_one_new;
                                layer_two = layer_two_new;
                                output_layer = output_layer_new;
                            }
                        }

                        //println!("Guess: {:.8}     vs Actual: {}", output_layer[0], training_results[i]);

                    //backprogagate errors
                        //get the "error gradients" (the error for each neuron, but altered based on the slope of the activation function)
                        //error_gradient    = error * gradient
                        //                  = (desired - guess) * derivative_of_sigmoid_at(node_val)
                        //                  = (desired - guess) * (guess * (1 - guess))
                        let mut error_gradients_output:Vec<f32> = Vec::new();
                            for i in 0..1 {
                                //                   (   actual           -     guess      ) *              slope
                                let error_gradient = (training_results[curr_train] - output_layer[i]) * output_layer[i] * (1.0 - output_layer[i]);
                                error_gradients_output.push(error_gradient);
                            }
                        //different since in the middle
                        //error_gradient    = error * gradient
                        //                  = weighted_sum(error_gradients_next_layer) * derivative_of_sigmoid_at(guess)
                        //                  = weighted_sum(error_gradients_next_layer) * guess * (1 - guess)
                        let mut error_gradients_two:Vec<f32> = Vec::new();
                            for i in 0..num_layer_two {
                                //sum up the errors coming from this point based on their weights
                                let mut sum_error_weights = 0.0;
                                for j in 0..1 {
                                    sum_error_weights += (weights_two_output[i])[j] * error_gradients_output[j];
                                }
                                //                             slope                     * "error" (combination of errors coming from this point, weighted)
                                let error_gradient = layer_two[i] * (1.0 - layer_two[i]) * sum_error_weights;
                                error_gradients_two.push(error_gradient);
                            }
                        let mut error_gradients_one:Vec<f32> = Vec::new();
                            for i in 0..num_layer_one {
                                //sum up the errors coming from this point based on their weights
                                let mut sum_error_weights = 0.0;
                                for j in 0..num_layer_two {
                                    sum_error_weights += (weights_one_two[i])[j] * error_gradients_two[j];
                                }
                                //                              slope                    * "error" (combination of errors coming from this point, weighted)
                                let error_gradient = layer_one[i] * (1.0 - layer_one[i]) * sum_error_weights;
                                error_gradients_one.push(error_gradient);
                            }

                    //update weights
                        //the change for any weight is "the source node's value" times "the error of the receiving node" (scaled by training_rate)
                        //change_in_weight[source][collector] = training_rate * neuron_value[source]  * error_gradient[collector]
                        //don't forget to update the bias
                        //for bias, just pretend that it's a node in the previous layer with value -1 and update its 'weights'
                        let training_rate:f32 = 0.5;

                        //input to one
                        match update_weights(   &training_samples[curr_train],
                                                weights_input_one, bias_one,
                                                &error_gradients_one,
                                                &training_rate)
                        {
                            Err(e) => {
                                panic!("Issue updating the weights from input to one:\n{}", e);
                            },
                            Ok((weights_input_one_new, bias_one_new)) => {
                                weights_input_one = weights_input_one_new;
                                bias_one = bias_one_new;
                            }
                        }
                        //one to two
                        match update_weights(   &layer_one,
                                                weights_one_two, bias_two,
                                                &error_gradients_two,
                                                &training_rate)
                        {
                            Err(e) => {
                                panic!("Issue updating the weights from one to two:\n{}", e);
                            },
                            Ok((weights_one_two_new, bias_two_new)) => {
                                weights_one_two = weights_one_two_new;
                                bias_two = bias_two_new;
                            }
                        }
                        //two to output
                        match update_weights(   &layer_two,
                                                weights_two_output, bias_output,
                                                &error_gradients_output,
                                                &training_rate)
                        {
                            Err(e) => {
                                panic!("Issue updating the weights from two to output:\n{}", e);
                            },
                            Ok((weights_two_output_new, bias_output_new)) => {
                                weights_two_output = weights_two_output_new;
                                bias_output = bias_output_new;
                            }
                        }

                } // for curr_train in size_training
            }// 'outer_loop -> the 3 passes

            //save the weights to a file
                let mut try_count = 0;
                while let Err(e) = write_to_file("mushroom_weights.txt",
                                                    &weights_input_one, &weights_one_two, &weights_two_output,
                                                    &bias_one, &bias_two, &bias_output){
                    println!("Attempted to save and failed");
                    println!("Due to: {:?}", e);
                    try_count += 1;
                    if try_count > 20 { break; }
                }
                if try_count <= 20 {
                    println!("Successfully saved to file");
                }
                

            println!("\n");

            //test on test data and print results
            let mut mean_square_difference = 0.0;
            let mut count_correct = 0;
            for i in 0..size_testing {
                match pass_through( &testing_samples[i],
                                    &weights_input_one, &bias_one,
                                    &weights_one_two, &bias_two,
                                    &weights_two_output, &bias_output)
                {
                    Err(e) => {   
                        println!("Something went wrong testing mushroom {}", i);
                        println!("Error: {}", e);
                        break;
                    }
                    Ok((_layer_one_new, _layer_two_new, output_layer_new)) =>{
                        output_layer = output_layer_new;
                    }
                }
                let diff = testing_results[i] - output_layer[0];
                mean_square_difference += diff * diff;

                if i < 200 {
                    if output_layer[0].round() == testing_results[i] {
                        count_correct += 1;
                        print!("Correct: ");
                    } else {
                        print!("       : ");
                    }

                    println!("Guess: {:.5} Actual: {}", output_layer[0], testing_results[i]);
                } else {
                    if output_layer[0].round() == testing_results[i] {
                        count_correct += 1;
                    }
                }
            }//for i in size_training

            mean_square_difference /= size_testing as f32;
            let percent_correct:f32 = (count_correct * 100) as f32 / size_testing as f32;

            println!("Mean square difference: {}", mean_square_difference);
            //since all 0's and 1's, completely correct is 0 and wrong is 1

            println!("Percent correct: {:.2}", percent_correct);

        }// Ok(contents)
    }//match on read
}//main