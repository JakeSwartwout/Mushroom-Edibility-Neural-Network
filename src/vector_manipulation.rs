use rand::Rng;

pub trait Randomize {
    fn fill_with_random(&mut self, inclusive_start: f32, exclusive_end: f32);
}

impl Randomize for Vec<f32>{
    fn fill_with_random(&mut self, inclusive_start: f32, exclusive_end: f32) {
        let mut generator = rand::thread_rng();
        for i in 0..self.len() {
            self[i] = generator.gen_range(inclusive_start, exclusive_end);
        }
    }
}

impl Randomize for Vec<Vec<f32>> {
    fn fill_with_random(&mut self, inclusive_start: f32, exclusive_end: f32) {
        for i in 0..self.len() {
            self[i].fill_with_random(inclusive_start, exclusive_end);
        }
    }
}




pub fn calculate_nodes(inputs: &Vec<f32>, weights: &Vec<Vec<f32>>, bias: &Vec<f32>) -> Result<Vec<f32>, String>{
    //for the weights, the outer vector correlates to the input neuron, the inner one to the outgoing weight
    
    //check the sizes
    if inputs.len() != weights.len() {
        return Err(String::from("Input size does not match the weights' input size"));
    }
    let input_length = inputs.len();

    if weights[0].len() != bias.len() {
        return Err(String::from("Weights' output size does not match bias size"));
    }
    let output_length = bias.len();
    
    //fill the vector
    let mut results: Vec<f32> = Vec::new();

    for out in 0..output_length {
        let mut sum:f32 = 0.0;
        
        for incoming in 0..input_length{
            sum += inputs[incoming] * (weights[incoming])[out];
        }

        results.push(activation(sum));
    }

    Ok(results)
}


pub fn activation(sum: f32) -> f32 {
    sigmoid(sum)
}

pub fn sigmoid(sum: f32) -> f32 {
    // 1 + e ^ (-x)
    let denominator = 1.0 + (-sum).exp();
    1.0 / denominator
}



pub fn pass_through(input: &Vec<f32>,
                    weights_input_one: &Vec<Vec<f32>>, bias_one: &Vec<f32>,
                    weights_one_two: &Vec<Vec<f32>>, bias_two: &Vec<f32>,
                    weights_two_output: &Vec<Vec<f32>>, bias_output: &Vec<f32>)
                            -> Result<(Vec<f32>, Vec<f32>, Vec<f32>), String>
{
    //layer one
    let layer_one = calculate_nodes(&input, &weights_input_one, &bias_one)?;
    //layer two
    let layer_two = calculate_nodes(&layer_one, &weights_one_two, &bias_two)?;
    //output
    let output = calculate_nodes(&layer_two, &weights_two_output, &bias_output)?;
    
    Ok((layer_one, layer_two, output))
}


//input to one
pub fn update_weights(  input: &Vec<f32>,
                        mut weights: Vec<Vec<f32>>, mut bias: Vec<f32>,
                        error_gradients: &Vec<f32>,
                        training_rate: &f32)
                -> Result<(Vec<Vec<f32>>, Vec<f32>), String>
{
    if input.len() != weights.len() {
        return Err(String::from("Input length != weights length"));
    }
    if bias.len() != weights[0].len() {
        return Err(String::from("Bias length != weights output length"));
    }
    if bias.len() != error_gradients.len() {
        return Err(String::from("Bias length != error gradients length"));
    }

    for source in 0..input.len() {
        for collector in 0..bias.len() {
            (weights[source])[collector] += training_rate * input[source] * error_gradients[collector];
        }
    }
        //the bias
        for collector in 0..bias.len() {
            bias[collector] += training_rate * -1.0 * error_gradients[collector];
        }

    Ok((weights, bias))
}
