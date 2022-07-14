# `exercise-06`
This exercise is about GUI programming using the [egui](https://docs.rs/egui/latest/egui/index.html) framework.
It is based on the previous exercise and should result in an GUI that visualizes the count of the unique words in a text box.

## ToDo
1. Run the example and get used to the way GUI works in egui.
2. Use the existing `count_words` function and the built-in egui components to implement the GUI without threading.
3. While the `count_words` function is running the GUI freezes. Move the function to a separate thread and only trigger its execution from the main thread. Use message passing.

## Resources
- https://docs.rs/egui/latest/egui/index.html
