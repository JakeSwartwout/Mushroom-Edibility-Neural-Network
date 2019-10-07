# Mushroom-Edibility-Neural-Network
A neural network built from scratch in the Rust programming language, which decides if a mushroom is poisonous or edible

I built this program to both learn Rust and learn how Neural Networks operate. The data is sourced from the UCI machine learning repository site. Specifically, [Here](https://archive.ics.uci.edu/ml/datasets/mushroom). It contains about 8000 mushroom descriptions, which are made up of selections in specific descriptive categories, such as shape, texture, color, and odor.

The program is a 4 layer network (consisting of an input, two hidden, and one output layer). I employ the sigmoid activation function to make back-propagation easier, and used [this](https://takinginitiative.wordpress.com/2008/04/03/basic-neural-network-tutorial-theory/) website to get the equations necessary to back-propagate (train the program).

I trained it on 75% of the data, which was about 6000 mushrooms. After passing through this three times, I then tested it against the remaining 25% of the data, or about 2000 mushrooms. It generally had an accuracy between 93 to 95%. I built it to store the weights and biases to compile the training, so within a short amount of time (likely about 20 total passes), I achieved an accuracy of 98.13%. The weights for this accuracy are stored in the file I uploaded (unless I change it and forget to update this, so sorry).

In the future, I plan to make a way to construct the network to check a single user-unputted mushroom (rather than train on the data set).
