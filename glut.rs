/* automatically generated by rust-bindgen */

import bindgen::{glutCreateWindow, glutDestroyWindow, glutDisplayFunc, glutInit};
import bindgen::{glutInitDisplayMode, glutPostRedisplay, glutSwapBuffers, glutTimerFunc};
import libc::*;
import dvec::{dvec, extensions};
import ptr::{addr_of, null};
import str::bytes;
import task::{local_data_get, local_data_set};
import unsafe::reinterpret_cast;
import vec::unsafe::to_ptr;

/* FIXME: global variable glutStrokeRoman */

/* FIXME: global variable glutStrokeMonoRoman */

/* FIXME: global variable glutBitmap9By15 */

/* FIXME: global variable glutBitmap8By13 */

/* FIXME: global variable glutBitmapTimesRoman10 */

/* FIXME: global variable glutBitmapTimesRoman24 */

/* FIXME: global variable glutBitmapHelvetica10 */

/* FIXME: global variable glutBitmapHelvetica12 */

/* FIXME: global variable glutBitmapHelvetica18 */

type GLenum = i32;
type GLint = i32;
type GLfloat = f32;
type GLdouble = f64;

enum Window = c_int;

const DOUBLE: c_uint = 2 as c_uint;

fn destroy<T>(-_value: ~[T]) {
    // let it drop
}

fn init() unsafe {
    let argc = 0 as c_int;
    let command = bytes("glut");
    let argv: (*u8, *u8) = (to_ptr(command), null());
    let argv_p = reinterpret_cast(addr_of(argv));

    glutInit(addr_of(argc), argv_p);

    destroy(command);
}

fn create_window(name: str) -> Window unsafe {
    let bytes = str::bytes(name);
    ret Window(glutCreateWindow(to_ptr(bytes) as *c_char));
}

fn destroy_window(window: Window) unsafe {
    glutDestroyWindow(*window);
}

fn display_callback_tls_key(+_callback: @fn@()) {
    // Empty.
}

extern fn display_callback() unsafe {
    let callback = local_data_get(display_callback_tls_key).get();
    (*callback)();
}

fn display_func(callback: fn@()) unsafe {
    local_data_set(display_callback_tls_key, @callback);
    glutDisplayFunc(display_callback);
}

fn timer_callback_tls_key(+_callback: @dvec<fn@()>) {
    // Empty.
}

extern fn timer_callback(index: int) unsafe {
    let callbacks = local_data_get(timer_callback_tls_key).get();
    ((*callbacks)[index as uint])();
}

fn timer_func(msecs: u32, callback: fn@()) unsafe {
    let callbacks;
    alt local_data_get(timer_callback_tls_key) {
        none {
            callbacks = @dvec();
            local_data_set(timer_callback_tls_key, copy callbacks);
        }
        some(existing_callbacks) {
            callbacks = existing_callbacks;
        }
    }

    callbacks.push(callback);
    let index = (callbacks.len() - 1) as c_int;
    glutTimerFunc(msecs, timer_callback, index);
}

#[cfg(target_os="macos")]
fn check_loop() unsafe {
    ext::glutCheckLoop();
}

#[cfg(target_os="linux")]
fn check_loop() unsafe {
    ext::glutMainLoopEvent();
}

fn init_display_mode(mode: c_uint) unsafe {
    glutInitDisplayMode(mode);
}

fn swap_buffers() unsafe {
    glutSwapBuffers();
}

fn post_redisplay() unsafe {
    glutPostRedisplay();
}

#[cfg(target_os="macos")]
#[nolink]
#[link_args="-framework GLUT"]
extern mod dummy {
}

#[cfg(target_os="linux")]
#[link_name="glut"]
extern mod dummy {
}

#[cfg(target_os="macos")]
#[nolink]
extern mod ext {
    // Mac GLUT extension.
    fn glutCheckLoop();
}

#[cfg(target_os="linux")]
#[nolink]
extern mod ext {
    // freeglut extension.
    fn glutMainLoopEvent();
}

#[nolink]
extern mod bindgen {

fn glutInit(++argcp: *c_int, ++argv: **c_char);

fn glutInitDisplayMode(++mode: c_uint);

fn glutInitDisplayString(++string: *c_char);

fn glutInitWindowPosition(++x: c_int, ++y: c_int);

fn glutInitWindowSize(++width: c_int, ++height: c_int);

fn glutMainLoop();

fn glutCreateWindow(++title: *c_char) -> c_int;

fn glutCreateSubWindow(++win: c_int, ++x: c_int, ++y: c_int, ++width: c_int, ++height: c_int) -> c_int;

fn glutDestroyWindow(++win: c_int);

fn glutPostRedisplay();

fn glutPostWindowRedisplay(++win: c_int);

fn glutSwapBuffers();

fn glutGetWindow() -> c_int;

fn glutSetWindow(++win: c_int);

fn glutSetWindowTitle(++title: *c_char);

fn glutSetIconTitle(++title: *c_char);

fn glutPositionWindow(++x: c_int, ++y: c_int);

fn glutReshapeWindow(++width: c_int, ++height: c_int);

fn glutPopWindow();

fn glutPushWindow();

fn glutIconifyWindow();

fn glutShowWindow();

fn glutHideWindow();

fn glutFullScreen();

fn glutSetCursor(++cursor: c_int);

fn glutWarpPointer(++x: c_int, ++y: c_int);

fn glutEstablishOverlay();

fn glutRemoveOverlay();

fn glutUseLayer(++layer: GLenum);

fn glutPostOverlayRedisplay();

fn glutPostWindowOverlayRedisplay(++win: c_int);

fn glutShowOverlay();

fn glutHideOverlay();

fn glutCreateMenu(++arg1: *u8) -> c_int;

fn glutDestroyMenu(++menu: c_int);

fn glutGetMenu() -> c_int;

fn glutSetMenu(++menu: c_int);

fn glutAddMenuEntry(++label: *c_char, ++value: c_int);

fn glutAddSubMenu(++label: *c_char, ++submenu: c_int);

fn glutChangeToMenuEntry(++item: c_int, ++label: *c_char, ++value: c_int);

fn glutChangeToSubMenu(++item: c_int, ++label: *c_char, ++submenu: c_int);

fn glutRemoveMenuItem(++item: c_int);

fn glutAttachMenu(++button: c_int);

fn glutDetachMenu(++button: c_int);

fn glutDisplayFunc(++func: *u8);

fn glutReshapeFunc(++func: *u8);

fn glutKeyboardFunc(++func: *u8);

fn glutMouseFunc(++func: *u8);

fn glutMotionFunc(++func: *u8);

fn glutPassiveMotionFunc(++func: *u8);

fn glutEntryFunc(++func: *u8);

fn glutVisibilityFunc(++func: *u8);

fn glutIdleFunc(++func: *u8);

fn glutTimerFunc(++millis: c_uint, ++func: *u8, ++value: c_int);

fn glutMenuStateFunc(++func: *u8);

fn glutSpecialFunc(++func: *u8);

fn glutSpaceballMotionFunc(++func: *u8);

fn glutSpaceballRotateFunc(++func: *u8);

fn glutSpaceballButtonFunc(++func: *u8);

fn glutButtonBoxFunc(++func: *u8);

fn glutDialsFunc(++func: *u8);

fn glutTabletMotionFunc(++func: *u8);

fn glutTabletButtonFunc(++func: *u8);

fn glutMenuStatusFunc(++func: *u8);

fn glutOverlayDisplayFunc(++func: *u8);

fn glutWindowStatusFunc(++func: *u8);

fn glutKeyboardUpFunc(++func: *u8);

fn glutSpecialUpFunc(++func: *u8);

fn glutJoystickFunc(++func: *u8, ++pollInterval: c_int);

fn glutSetColor(++arg1: c_int, ++red: GLfloat, ++green: GLfloat, ++blue: GLfloat);

fn glutGetColor(++ndx: c_int, ++component: c_int) -> GLfloat;

fn glutCopyColormap(++win: c_int);

fn glutGet(++_type: GLenum) -> c_int;

fn glutDeviceGet(++_type: GLenum) -> c_int;

fn glutExtensionSupported(++name: *c_char) -> c_int;

fn glutGetModifiers() -> c_int;

fn glutLayerGet(++_type: GLenum) -> c_int;

fn glutGetProcAddress(++procName: *c_char) -> *c_void;

fn glutBitmapCharacter(++font: *c_void, ++character: c_int);

fn glutBitmapWidth(++font: *c_void, ++character: c_int) -> c_int;

fn glutStrokeCharacter(++font: *c_void, ++character: c_int);

fn glutStrokeWidth(++font: *c_void, ++character: c_int) -> c_int;

fn glutBitmapLength(++font: *c_void, ++string: *c_uchar) -> c_int;

fn glutStrokeLength(++font: *c_void, ++string: *c_uchar) -> c_int;

fn glutWireSphere(++radius: GLdouble, ++slices: GLint, ++stacks: GLint);

fn glutSolidSphere(++radius: GLdouble, ++slices: GLint, ++stacks: GLint);

fn glutWireCone(++base: GLdouble, ++height: GLdouble, ++slices: GLint, ++stacks: GLint);

fn glutSolidCone(++base: GLdouble, ++height: GLdouble, ++slices: GLint, ++stacks: GLint);

fn glutWireCube(++size: GLdouble);

fn glutSolidCube(++size: GLdouble);

fn glutWireTorus(++innerRadius: GLdouble, ++outerRadius: GLdouble, ++sides: GLint, ++rings: GLint);

fn glutSolidTorus(++innerRadius: GLdouble, ++outerRadius: GLdouble, ++sides: GLint, ++rings: GLint);

fn glutWireDodecahedron();

fn glutSolidDodecahedron();

fn glutWireTeapot(++size: GLdouble);

fn glutSolidTeapot(++size: GLdouble);

fn glutWireOctahedron();

fn glutSolidOctahedron();

fn glutWireTetrahedron();

fn glutSolidTetrahedron();

fn glutWireIcosahedron();

fn glutSolidIcosahedron();

fn glutVideoResizeGet(++param: GLenum) -> c_int;

fn glutSetupVideoResizing();

fn glutStopVideoResizing();

fn glutVideoResize(++x: c_int, ++y: c_int, ++width: c_int, ++height: c_int);

fn glutVideoPan(++x: c_int, ++y: c_int, ++width: c_int, ++height: c_int);

fn glutReportErrors();

fn glutIgnoreKeyRepeat(++ignore: c_int);

fn glutSetKeyRepeat(++repeatMode: c_int);

fn glutForceJoystickFunc();

fn glutGameModeString(++string: *c_char);

fn glutEnterGameMode() -> c_int;

fn glutLeaveGameMode();

fn glutGameModeGet(++mode: GLenum) -> c_int;

}
