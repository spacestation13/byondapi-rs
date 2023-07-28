
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

	if(x != "awameow")
		throw EXCEPTION("Pointer read/write failed")

/obj/proc/get_name()
	return name

/test/proc/test_proc_call()
	var/obj/O = new()
	O.name = "test name"

	var/ret = call_ext("byondapi_test.dll", "byond:test_proc_call")(O)

	if(O.name != ret)
		throw EXCEPTION("Call proc failed, expected rust to return 'test name' but got '[ret]'")

/test/proc/test_readwrite_var()
	var/obj/O = new()
	O.name = "test name"

	var/ret = call_ext("byondapi_test.dll", "byond:test_readwrite_var")(O)

	if(O.name != ret)
		throw EXCEPTION("Call proc failed, expected rust to return 'test name' but got '[ret]'")

/test/proc/test_list()
	var/list/L = list(1, 2, 3, 4, 5, 6, 7)

	var/list/ret = call_ext("byondapi_test.dll", "byond:test_list_push")(L)

	if(!islist(ret) || ret[8] != 8)
		throw EXCEPTION("List push failed")

	var/list/doubled = call_ext("byondapi_test.dll", "byond:test_list_double")(L)
	if(!islist(doubled) || doubled[3] != 6)
		throw EXCEPTION("List iter failed")