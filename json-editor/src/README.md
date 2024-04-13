*This follows a common approach to small applications in `ratatui`, where we have a state file, a UI file, and the main file to tie it all together.*

## Application modes

It is useful to think about the several “modes” that your application can be in. Thinking in “modes” will make it easier to segregate everything from what window is getting drawn, to what keybinds to listen for.

We will be using the application’s state to track two things:

1. what screen the user is seeing,
2. which box should be highlighted, the “key” or “value” (this only applies when the user is editing a key-value pair).
