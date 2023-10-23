#ifndef BYONDAPI_H
#define BYONDAPI_H

/*
	BYOND public API version 515.1609

	Because for some reason nobody can get their ducks in a row, all of the
	exported functions from byondcore.dll/libbyond.so are limited to C
	conventions only. A header and source file for C++ wrappers is available
	for inclusion in your projects.
 */

#if defined(WIN32) || defined(WIN64)
#define IS_WINDOWS
#else
#define IS_LINUX
#endif


// See https://github.com/cpredef/predef/blob/master/Architectures.md
#if defined(i386) || defined(__i386) || defined(__i386__) || defined(_M_IX86) || defined(_X86_) || defined(__X86__)
#define _X86
#define _X86ORX64
#define DM_32BIT
#elif defined(__amd64__) || defined(__amd64) || defined(__x86_64__) || defined(__x86_64) || defined(_M_AMD64) || defined(_M_X64) || defined(_WIN64) || defined(WIN64)
#define _X64
#define _X86ORX64
#define DM_64BIT
#elif defined(__arm__) || defined(_M_ARM)
#define _ARM
#if defined(__LP64__) || defined(_LP64)
#define DM_64BIT
#else
#define DM_32BIT
#endif
#endif



/*types*/
typedef unsigned char  u1c;
typedef signed   char  s1c;
typedef unsigned short u2c;
typedef signed   short s2c;
#ifdef DM_64BIT
typedef unsigned int   u4c;
typedef signed   int   s4c;
#else
typedef unsigned long  u4c;
typedef signed   long  s4c;
#endif

#if defined(_MSC_VER) || defined(__BORLANDC__)
  typedef __int64 s8c;
  typedef unsigned __int64 u8c;
#else
  typedef long long int s8c;
  typedef unsigned long long int u8c;
#endif

union u4cOrPointer {
	u4c num;
	void *ptr;
};

#ifdef __GNUC__
#define GCC_VERSION (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)
#else
#define GCC_VERSION 0
#endif


// determine if move-constructor and move-assignment are supported
#if defined(_MSC_VER) && _MSC_VER >= 1800
#define _SUPPORT_MOVES
#elif defined(__GNUC__) && GCC_VERSION >= 50000
#define _SUPPORT_MOVES
#endif


#define u1cMASK ((u1c)0xff)
#define u2cMASK ((u2c)0xffff)
#define u3cMASK ((u4c)0xffffffL)
#define u4cMASK ((u4c)0xffffffffL)

#define u1cMAX u1cMASK
#define u2cMAX u2cMASK
#define u3cMAX u3cMASK
#define u4cMAX u4cMASK

#define s1cMAX 0x7f
#define s1cMIN (-0x7f)
#define s2cMAX 0x7fff
#define s2cMIN (-0x7fff)
#define s4cMAX 0x7fffffffL
#define s4cMIN (-0x7fffffffL)

#define NONE u2cMAX
#define NOCH u1cMAX

/* dll export stuff */
#ifdef WIN32
#define DUNGPUB    __declspec(dllimport)
#define BYOND_EXPORT __declspec(dllexport)	// for functions in user-defined DLLs for use with call_ext()
#else // unix/g++, combine with -fvisibility=hidden to hide non-exported symbols
#define DUNGPUB
#define BYOND_EXPORT __attribute__ ((visibility("default")))	// for functions in user-defined .so libraries for use with call_ext()
#endif


// ByondValue

/*
	Many of the routines in this library return a bool value. If true, the
	operation succeeded. If false, it failed and calling Byond_LastError() will
	return an error string.

	The C++ wrappers change this true/false behavior to raise exceptions instead.
	This would be preferable across the board but throwing exceptions across
	module boundaries is bad juju.
 */

extern "C" {

DUNGPUB char const *Byond_LastError();
DUNGPUB void Byond_GetVersion(u4c *version, u4c *build);

typedef u1c ByondValueType;
union ByondValueData {
	u4c ref;
	float num;
	char *str;
};

/*
	You MUST call one of the ByondValue_Init routines before using this
	structure.

	You SHOULD call ByondValue_Free() to clean this structure up, but it's
	only technically needed to clean up strings. For safety's sake, just be
	sure to clean it up.
 */
struct CByondValue {
	ByondValueType type;
	u1c junk1, junk2, junk3;	// padding
	ByondValueData data;
};

DUNGPUB void ByondValue_Init(CByondValue *v);
DUNGPUB void ByondValue_InitNum(CByondValue *v, float num);
DUNGPUB bool ByondValue_InitStr(CByondValue *v, char const *str);
DUNGPUB void ByondValue_InitRef(CByondValue *v, ByondValueType type, u4c ref);
DUNGPUB void ByondValue_Free(CByondValue *v);

DUNGPUB void ByondValue_CopyFrom(CByondValue *dst, CByondValue const *src);	// frees dst's old value
DUNGPUB void ByondValue_MoveFrom(CByondValue *dst, CByondValue *src);	// frees src after copy, and frees dst's old value

DUNGPUB ByondValueType ByondValue_Type(CByondValue const *v);
DUNGPUB bool ByondValue_IsNull(CByondValue const *v);
DUNGPUB bool ByondValue_IsNum(CByondValue const *v);
DUNGPUB bool ByondValue_IsStr(CByondValue const *v);
DUNGPUB bool ByondValue_IsList(CByondValue const *v);

DUNGPUB float ByondValue_GetNum(CByondValue const *v);
DUNGPUB char const *ByondValue_GetStr(CByondValue const *v);
DUNGPUB u4c ByondValue_GetRef(CByondValue const *v);

DUNGPUB void ByondValue_SetNum(CByondValue *v, float f);
DUNGPUB bool ByondValue_SetStr(CByondValue *v, char const *str);
DUNGPUB void ByondValue_SetRef(CByondValue *v, ByondValueType type, u4c ref);

DUNGPUB bool ByondValue_Equals(CByondValue const *a, CByondValue const *b);

/*
	You MUST call one of the ByondValueList_Init routines before using this
	structure, and you MUST call ByoundValueList_Free() to free it.

	Routines will return false if allocation fails.
 */
struct CByondValueList {
	CByondValue *items;
	u4c count, capacity;
};

DUNGPUB void ByondValueList_Init(CByondValueList *list);
DUNGPUB bool ByondValueList_InitCount(CByondValueList *list, u4c count);
DUNGPUB void ByondValueList_Free(CByondValueList *list);

DUNGPUB bool ByondValueList_CopyFrom(CByondValueList *dst, CByondValueList const *src);
DUNGPUB void ByondValueList_MoveFrom(CByondValueList *dst, CByondValueList *src);

DUNGPUB bool ByondValueList_SetCount(CByondValueList *list, u4c count);	// will grow capacity if needed, preferring 25% increase
DUNGPUB bool ByondValueList_SetCapacity(CByondValueList *list, u4c capacity);
DUNGPUB bool ByondValueList_Add(CByondValueList *list, CByondValue const *v);
DUNGPUB bool ByondValueList_InsertAt(CByondValueList *list, int idx, CByondValue const *v);	// idx < 0 means end of list
DUNGPUB bool ByondValueList_Splice(CByondValueList *list, int idx, u4c delete_count, CByondValue const *v=0, u4c insert_count=0);	// idx < 0 means end of list
DUNGPUB u4c ByondValueList_RemoveAt(CByondValueList *list, u4c idx, u4c n=1);	// returns # of removed items

// Other useful structs

struct CByondXYZ {
	s2c x, y, z, junk;
};

/*
	In the following functions, anything that fills a result value (e.g.,
	ReadVar, CallProc) will create a temporary reference to the value. So if
	the result is an object or list or such, it will remain valid until the
	end of the current tick unless something explicitly deletes it. You can
	also let go of the temporary reference early by calling Byond_DecRef().

	If the validity of a reference is ever in doubt, call Byond_TestRef().


	Thread safety:

	Anything that requires reading will block if called outside of the main
	thread. Write operations on the wrong thread will not block, but they will
	schedule for later, return immediately, and not report failure.
 */

DUNGPUB u4c Byond_GetStrId(char const *str);	// does not add a string to the tree if not found; returns NONE if no string match

DUNGPUB bool Byond_ReadVar(CByondValue const *loc, char const *varname, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_ReadVarByStrId(CByondValue const *loc, u4c varname, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_WriteVar(CByondValue const *loc, char const *varname, CByondValue const *val);
DUNGPUB bool Byond_WriteVarByStrId(CByondValue const *loc, u4c varname, CByondValue const *val);

DUNGPUB bool Byond_CreateList(CByondValue *result);	// result MUST be initialized first!

DUNGPUB bool Byond_ReadList(CByondValue const *loc, CByondValueList *list);	// list MUST be initialized first!
DUNGPUB bool Byond_WriteList(CByondValue const *loc, CByondValueList const *list);

DUNGPUB bool Byond_ReadListIndex(CByondValue const *loc, CByondValue const *idx, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_WriteListIndex(CByondValue const *loc, CByondValue const *idx, CByondValue const *val);

DUNGPUB bool Byond_ReadPointer(CByondValue const *ptr, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_WritePointer(CByondValue const *ptr, CByondValue const *val);

/*
	Proc calls:

	arg is an array of arguments; can be null arg_count is 0.

	The call is implicitly a waitfor=0 call; if the callee sleeps it will return
	immediately and finish later.

	If called in the wrong thread, it will spawn() the proc and return null.
 */
DUNGPUB bool Byond_CallProc(CByondValue const *src, char const *name, CByondValue const *arg, u4c arg_count, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_CallProcByStrId(CByondValue const *src, u4c name, CByondValue const *arg, u4c arg_count, CByondValue *result);	// result MUST be initialized first!

DUNGPUB bool Byond_CallGlobalProc(char const *name, CByondValue const *arg, u4c arg_count, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_CallGlobalProcByStrId(u4c name, CByondValue const *arg, u4c arg_count, CByondValue *result);	// result MUST be initialized first!

// Use BYOND's internal value-to-text conversion
DUNGPUB bool Byond_ToString(CByondValue const *src, CByondValue *result);	// result MUST be initialized first!

// Other builtins
DUNGPUB bool Byond_Block(CByondXYZ const *corner1, CByondXYZ const *corner2, CByondValueList *result);	// result MUST be initialized first!
DUNGPUB bool Byond_Length(CByondValue const *src, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_LocateIn(CByondValue const *type, CByondValue const *list, CByondValue *result);	// result MUST be initialized first!; list may be a null pointer
DUNGPUB bool Byond_LocateXYZ(CByondXYZ const *xyz, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_New(CByondValue const *type, CByondValue const *argA, u4c argS, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_NewArglist(CByondValue const *type, CByondValue const *arglist, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_Refcount(CByondValue const *type, CByondValue *result);	// result MUST be initialized first!
DUNGPUB bool Byond_XYZ(CByondValue const *src, CByondXYZ *xyz);	// still returns true if the atom is off-map, but xyz will be 0,0,0

/*
	Generally you don't want to mess with inc/decref calls, except that for
	temporary references you can use Byond_DecRef() to let go of the temporary
	reference.

	Call ByondValue_IncRef() to create a permanent reference to an object
	within Byondapi. It will exist until ByondValue_DecRef() removes it or the
	object is hard-deleted.

	ByondValue_DecRef() will remove a permanent reference created by Byondapi.
	If there is no permanent reference, it will remove any temporary
	reference that was set to expire when the tick ends. If Byondapi has no
	references to the object, the call will be ignored.

	These only apply to ref types, not null/num/string. Any runtime errors
	caused by decref (if the object is deleted or another object ends up
	getting deleted as an indirect result) are ignored.
 */
DUNGPUB void ByondValue_IncRef(CByondValue const *src);
DUNGPUB void ByondValue_DecRef(CByondValue const *src);

// Returns true if the ref is valid.
// Returns false if the ref was not valid and had to be changed to null.
// This only applies to ref types, not null/num/string which are always valid.
DUNGPUB bool Byond_TestRef(CByondValue *src);

};	// extern "C"



#endif	// BYONDAPI_H
