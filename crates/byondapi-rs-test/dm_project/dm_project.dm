
/world/New()
	world.log = file("dd_log.txt")

	for(var/func in typesof(/test/proc))
		world.log << "[func] [copytext("------------------------------------------------------------------------", length("[func]"))]"
		call(new /test, func)()

	del(src)

/test/proc/test_connection()
	var/ret = call_ext("byondapi_test.dll", "byond:test_connection")()
	if (ret != 69)
		throw EXCEPTION("Connection bad")

/test/proc/test_args()
	var/obj/O = new()
	O.name = "meow"
	var/obj/ret = call_ext("byondapi_test.dll", "byond:test_args")(O)
	
	if (ret.name != O.name)
		throw EXCEPTION("Object did not make it through FFI")

/test/proc/send_test()
	call_ext("byondapi_test.dll", "byond:send_test")()


/test/proc/test_ptr()
	var/x = "meow"
	var/ptr = &x

	call_ext("byondapi_test.dll", "byond:test_ptr")(ptr)

	// if(x != "awameow")
	// 	throw EXCEPTION("Pointer read/write failed")

/obj/proc/get_name()
	world.log << "get_name"
	return name

/test/proc/test_proc_call()
	var/obj/O = new()
	O.name = "test name"

	var/ret = call_ext("byondapi_test.dll", "byond:test_proc_call")(O)

	if(O.name != ret)
		throw EXCEPTION("Call proc failed, expected rust to return 'test name' but got '[ret]'")
