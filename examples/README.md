# Examples

## Source PNGs

The PNGs used as sources for the different examples are:

![panel](../assets/glassPanel_corners.png)
![button](../assets/blue_button02.png)
![corner panel](../assets/metalPanel_yellowCorner.png)

All assets are created by [Kenney](https://www.kenney.nl).

## As a plugin

### [plugin.rs](https://github.com/vleue/bevy_ninepatch/blob/main/examples/plugin.rs)

Adding a simple 9-Patch UI element by using the `NinePatchBundle` component bundle.

![example with plugin](./plugin.png)

### [change_size.rs](https://github.com/vleue/bevy_ninepatch/blob/main/examples/change_size.rs)

Changing the size of a 9-Patch UI element by modifying the `Style.size` component.

![changing size of component](./change_size.gif)

### [multi_content_with_content_map.rs](https://github.com/vleue/bevy_ninepatch/blob/main/examples/multi_content_with_content_map.rs)

To set the content of a 9-Patch UI element, you can specify an `Entity` when setting the `NinePatchData` component. This `Entity` will be set as a children of the 9-Patch UI element.

![setting several contents](./multi_content.png)

### [multi_content_with_system.rs](https://github.com/vleue/bevy_ninepatch/blob/main/examples/multi_content_with_content_map.rs)

When more flexibility is needed to set the content, it can be done by creating a system with a query on the `NinePatchContent` component.

### [full.rs](https://github.com/vleue/bevy_ninepatch/blob/main/examples/full.rs)

Complete example with:
* a complex 9-Patch UI element with two different content zones and a top bar that has two different parts that can grow
* 9-Patch UI elements inside a 9-Patch UI element inside a 9-Patch UI element
* Some 9-Patch UI elements change size during time

![full example](./full.gif)
