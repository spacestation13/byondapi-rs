/*
	These are simple defaults for your project.
 */

world
	fps = 25		// 25 frames per second
	icon_size = 32	// 32x32 icon size by default

	view = 6		// show up to 6 tiles outward from center (13x13 view)


// Make objects move 8 pixels per tick when walking

mob
	step_size = 8

obj
	step_size = 8

/world/New()
	. = ..()
	
	var/a=3, b=4
	var/p = &a
	world.log << *p   // same as world << a
	*p = 5    // same as a = 5
	world.log << *p
	var/ret = call_ext("fakelib.dll", "byond:test")(p)

	world.log << "number: [*p]"
	world.log << "ret: [ret]"
