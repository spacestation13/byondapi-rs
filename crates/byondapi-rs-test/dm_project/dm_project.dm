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

	world.log << "## SENDING OBJECT ##"
	send_obj()

	world.log << "## POINTER BUG ##"
	ptr_bug()

	world.log << "## LIST TEST ##"
	test_list()

/obj/proc/testproc()
	world.log << "hi test proc on obj!"

/proc/send_obj()
	var/obj/O = new()
	O.name = "meow"
	var/list/ret = call_ext("fakelib.dll", "byond:test_obj")(O)
	world.log << "ret: [json_encode(ret)]"


/proc/ptr_bug()
	var/a=3, b=4
	var/p = &a
	var/p2 = &a
	world.log << *p   // same as world << a
	*p = 5    // same as a = 5
	world.log << *p
	var/ret = call_ext("fakelib.dll", "byond:test_ptr")(p2) // runtime error: bad pointer

	// (if test_ptr is stubbed out we can check 
	world.log << "number: [*p]" 
	world.log << "ret: [ret]"

/proc/test_list()
	var/list/L = list(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
	var/ret = call_ext("fakelib.dll", "byond:test_list")(L)
	world.log << "list: [json_encode(L)]"
	world.log << "ret: [json_encode(ret)]"