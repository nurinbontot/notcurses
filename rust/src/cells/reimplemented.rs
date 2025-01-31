//! `cell*_*` reimplemented functions.

use libc::strcmp;

use crate::{
    cstring, nccell_release, rstring, NcAlphaBits, NcCell, NcChannel, NcChannels, NcComponent,
    NcIntResult, NcPaletteIndex, NcPlane, NcRgb, NcStyle, NCALPHA_BGDEFAULT_MASK,
    NCALPHA_BG_PALETTE, NCALPHA_FGDEFAULT_MASK, NCALPHA_FG_PALETTE, NCALPHA_OPAQUE, NCRESULT_ERR,
    NCRESULT_OK, NCSTYLE_MASK,
};

// Alpha -----------------------------------------------------------------------

/// Extracts the foreground [`NcAlphaBits`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[fg_alpha()][NcCell#method.fg_alpha].*
#[inline]
pub fn nccell_fg_alpha(cell: &NcCell) -> NcAlphaBits {
    crate::ncchannels_fg_alpha(cell.channels)
}

/// Extracts the background [`NcAlphaBits`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[bg_alpha()][NcCell#method.bg_alpha].*
#[inline]
pub fn nccell_bg_alpha(cell: &NcCell) -> NcAlphaBits {
    crate::ncchannels_bg_alpha(cell.channels)
}

/// Sets the foreground [`NcAlphaBits`] of an [`NcCell`].
///
/// *Method: NcCell.[set_fg_alpha()][NcCell#method.set_fg_alpha].*
#[inline]
pub fn nccell_set_fg_alpha(cell: &mut NcCell, alpha: NcAlphaBits) {
    crate::ncchannels_set_fg_alpha(&mut cell.channels, alpha);
}

/// Sets the background [`NcAlphaBits`] of an [`NcCell`].
///
/// *Method: NcCell.[set_bg_alpha()][NcCell#method.set_bg_alpha].*
#[inline]
pub fn nccell_set_bg_alpha(cell: &mut NcCell, alpha: NcAlphaBits) {
    crate::ncchannels_set_bg_alpha(&mut cell.channels, alpha);
}

// NcComponent ---------------------------------------------------------------------

/// Gets the foreground [`NcComponent`]s of an [`NcCell`],
/// and returns the [`NcChannel`] (which can have some extra bits set).
///
/// *Method: NcCell.[fg_rgb8()][NcCell#method.fg_rgb8].*
#[inline]
pub fn nccell_fg_rgb8(
    cell: &NcCell,
    red: &mut NcComponent,
    green: &mut NcComponent,
    blue: &mut NcComponent,
) -> NcChannel {
    crate::ncchannels_fg_rgb8(cell.channels, red, green, blue)
}

/// Gets the background [`NcComponent`]s of an [`NcCell`],
/// and returns the [`NcChannel`] (which can have some extra bits set).
///
/// *Method: NcCell.[bg_rgb8()][NcCell#method.bg_rgb8].*
#[inline]
pub fn nccell_bg_rgb8(
    cell: &NcCell,
    red: &mut NcComponent,
    green: &mut NcComponent,
    blue: &mut NcComponent,
) -> NcChannel {
    crate::ncchannels_bg_rgb8(cell.channels, red, green, blue)
}

/// Sets the foreground [`NcComponent`]s of the [`NcCell`],
/// and marks it as not using the "default color".
///
/// *Method: NcCell.[set_fg_rgb8()][NcCell#method.set_fg_rgb8].*
#[inline]
pub fn nccell_set_fg_rgb8(
    cell: &mut NcCell,
    red: NcComponent,
    green: NcComponent,
    blue: NcComponent,
) {
    crate::ncchannels_set_fg_rgb8(&mut cell.channels, red, green, blue);
}

/// Sets the background [`NcComponent`]s of the [`NcCell`],
/// and marks it as not using the "default color".
///
/// *Method: NcCell.[set_bg_rgb8()][NcCell#method.set_bg_rgb8].*
#[inline]
pub fn nccell_set_bg_rgb8(
    cell: &mut NcCell,
    red: NcComponent,
    green: NcComponent,
    blue: NcComponent,
) {
    crate::ncchannels_set_bg_rgb8(&mut cell.channels, red, green, blue);
}

// NcRgb -----------------------------------------------------------------------

/// Gets the foreground [`NcRgb`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[fg_rgb()][NcCell#method.fg_rgb].*
#[inline]
pub fn nccell_fg_rgb(cell: &NcCell) -> NcRgb {
    crate::ncchannels_fg_rgb(cell.channels)
}

/// Gets the background [`NcRgb`] from an [`NcCell`] (shifted to LSBs).
///
/// *Method: NcCell.[bg_rgb()][NcCell#method.bg_rgb].*
#[inline]
pub fn nccell_bg_rgb(cell: &NcCell) -> NcRgb {
    crate::ncchannels_bg_rgb(cell.channels)
}

/// Sets the foreground [`NcRgb`] of an [`NcCell`],
/// and marks it as not using the default color.
///
/// *Method: NcCell.[set_fg_rgb()][NcCell#method.set_fg_rgb].*
#[inline]
pub fn nccell_set_fg_rgb(cell: &mut NcCell, rgb: NcRgb) {
    crate::ncchannels_set_fg_rgb(&mut cell.channels, rgb);
}

/// Sets the background [`NcRgb`] of an [`NcCell`],
/// and marks it as not using the default color.
///
/// *Method: NcCell.[set_bg_rgb()][NcCell#method.set_bg_rgb].*
#[inline]
pub fn nccell_set_bg_rgb(cell: &mut NcCell, rgb: NcRgb) {
    crate::ncchannels_set_bg_rgb(&mut cell.channels, rgb);
}

// Default ---------------------------------------------------------------------

/// Indicates to use the "default color" for the foreground [`NcChannel`]
/// of an [`NcCell`].
///
/// *Method: NcCell.[set_fg_default()][NcCell#method.set_fg_default].*
#[inline]
pub fn nccell_set_fg_default(cell: &mut NcCell) {
    crate::ncchannels_set_fg_default(&mut cell.channels);
}

/// Indicates to use the "default color" for the background [`NcChannel`]
/// of an [`NcCell`].
///
/// *Method: NcCell.[set_bg_default()][NcCell#method.set_bg_default].*
#[inline]
pub fn nccell_set_bg_default(cell: &mut NcCell) {
    crate::ncchannels_set_bg_default(&mut cell.channels);
}

/// Is the foreground [`NcChannel`] of this [`NcCell`] using the
/// "default foreground color"?
///
/// *Method: NcCell.[fg_default_p()][NcCell#method.fg_default_p].*
#[inline]
pub fn nccell_fg_default_p(cell: &NcCell) -> bool {
    crate::ncchannels_fg_default_p(cell.channels)
}

/// Is the background [`NcChannel`] of this [`NcCell`] using the
/// "default background color"?
///
/// The "default background color" must generally be used to take advantage of
/// terminal-effected transparency.
///
/// *Method: NcCell.[bg_default_p()][NcCell#method.bg_default_p].*
#[inline]
pub fn nccell_bg_default_p(cell: &NcCell) -> bool {
    crate::ncchannels_bg_default_p(cell.channels)
}

// Palette ---------------------------------------------------------------------

/// Is the foreground [`NcChannel`] of this [`NcCell`] using an
/// [`NcPaletteIndex`] indexed [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcCell.[fg_palindex_p()][NcCell#method.fg_palindex_p].*
#[inline]
pub fn nccell_fg_palindex_p(cell: &NcCell) -> bool {
    crate::ncchannels_fg_palindex_p(cell.channels)
}

/// Is the background [`NcChannel`] of this [`NcCell`] using an
/// [`NcPaletteIndex`] indexed [`NcPalette`][crate::NcPalette] color?
///
/// *Method: NcCell.[bg_palindex_p()][NcCell#method.bg_palindex_p].*
#[inline]
pub fn nccell_bg_palindex_p(cell: &NcCell) -> bool {
    crate::ncchannels_bg_palindex_p(cell.channels)
}

/// Gets the [`NcPaletteIndex`] of the foreground [`NcChannel`] of the [`NcCell`].
///
/// *Method: NcCell.[fg_palindex()][NcCell#method.fg_palindex].*
#[inline]
#[allow(clippy::unnecessary_cast)]
pub const fn nccell_fg_palindex(cell: &NcCell) -> NcPaletteIndex {
    ((cell.channels & 0xff00000000 as NcChannels) >> 32) as NcPaletteIndex
}

/// Gets the [`NcPaletteIndex`] of the background [`NcChannel`] of the [`NcCell`].
///
/// *Method: NcCell.[bg_palindex()][NcCell#method.bg_palindex].*
#[inline]
#[allow(clippy::unnecessary_cast)]
pub const fn nccell_bg_palindex(cell: &NcCell) -> NcPaletteIndex {
    (cell.channels & 0xff) as NcPaletteIndex
}

/// Sets an [`NcCell`]'s foreground [`NcPaletteIndex`].
///
/// Also sets [NCALPHA_FG_PALETTE] and [NCALPHA_OPAQUE],
/// and clears out [NCALPHA_FGDEFAULT_MASK].
///
/// *Method: NcCell.[set_fg_palindex()][NcCell#method.set_fg_palindex].*
//
// NOTE: unlike the original C function, this one can't fail
#[inline]
#[allow(clippy::unnecessary_cast)]
pub fn nccell_set_fg_palindex(cell: &mut NcCell, index: NcPaletteIndex) {
    cell.channels |= NCALPHA_FGDEFAULT_MASK;
    cell.channels |= NCALPHA_FG_PALETTE;
    nccell_set_fg_alpha(cell, NCALPHA_OPAQUE);
    cell.channels &= 0xff000000ffffffff as NcChannels;
    cell.channels |= (index as NcChannels) << 32;
}

/// Sets an [`NcCell`]'s background [`NcPaletteIndex`].
///
/// Also sets [`NCALPHA_BG_PALETTE`] and [`NCALPHA_OPAQUE`],
/// and clears out [`NCALPHA_BGDEFAULT_MASK`].
///
/// *Method: NcCell.[set_bg_palindex()][NcCell#method.set_bg_palindex].*
//
// NOTE: unlike the original C function, this one can't fail
#[inline]
pub fn nccell_set_bg_palindex(cell: &mut NcCell, index: NcPaletteIndex) {
    cell.channels |= NCALPHA_BGDEFAULT_MASK as NcChannels;
    cell.channels |= NCALPHA_BG_PALETTE as NcChannels;
    nccell_set_bg_alpha(cell, NCALPHA_OPAQUE);
    cell.channels &= 0xffffffffff000000;
    cell.channels |= index as NcChannels;
}

// Styles ----------------------------------------------------------------------

/// Extracts the [`NcStyle`] bits from an [`NcCell`].
///
/// *Method: NcCell.[cell_styles()][NcCell#method.cell_styles].*
#[inline]
pub const fn nccell_styles(cell: &NcCell) -> NcStyle {
    cell.stylemask
}

/// Adds the specified [`NcStyle`] bits to an [`NcCell`]'s existing spec.,
/// whether they're actively supported or not.
///
/// *Method: NcCell.[styles_on()][NcCell#method.styles_on].*
#[inline]
pub fn nccell_on_styles(cell: &mut NcCell, stylebits: NcStyle) {
    cell.stylemask |= stylebits & NCSTYLE_MASK as u16;
}

/// Removes the specified [`NcStyle`] bits from an [`NcCell`]'s existing spec.
///
/// *Method: NcCell.[styles_off()][NcCell#method.styles_off].*
#[inline]
pub fn nccell_off_styles(cell: &mut NcCell, stylebits: NcStyle) {
    cell.stylemask &= !(stylebits & NCSTYLE_MASK as u16);
}

/// Sets *just* the specified [`NcStyle`] bits for an [`NcCell`],
/// whether they're actively supported or not.
///
/// *Method: NcCell.[styles_set()][NcCell#method.styles_set].*
#[inline]
pub fn nccell_set_styles(cell: &mut NcCell, stylebits: NcStyle) {
    cell.stylemask = stylebits & NCSTYLE_MASK as u16;
}

// Chars -----------------------------------------------------------------------

/// Returns the number of columns occupied by 'c'. see ncstrwidth() for an
/// equivalent for multiple EGCs.
#[inline]
pub const fn nccell_cols(cell: &NcCell) -> u8 {
    if cell.width != 0 {
        cell.width
    } else {
        1
    }
}

#[deprecated]
pub fn nccell_width(plane: &NcPlane, cell: &NcCell) -> NcIntResult {
    unsafe { crate::ffi::nccell_width(plane, cell) }
}

/// Does the [`NcCell`] contain an East Asian Wide codepoint?
///
/// *Method: NcCell.[double_wide_p()][NcCell#method.double_wide_p].*
#[inline]
pub const fn nccell_double_wide_p(cell: &NcCell) -> bool {
    cell.width > 0
}

/// Is this the right half of a wide character?
///
/// *Method: NcCell.[wide_right_p()][NcCell#method.wide_right_p].*
#[inline]
pub const fn nccell_wide_right_p(cell: &NcCell) -> bool {
    nccell_double_wide_p(cell) && cell.gcluster == 0
}

/// Is this the left half of a wide character?
///
/// *Method: NcCell.[wide_left_p()][NcCell#method.wide_left_p].*
#[inline]
pub const fn nccell_wide_left_p(cell: &NcCell) -> bool {
    nccell_double_wide_p(cell) && cell.gcluster != 0
}

// /// Loads a 7-bit `EGC` character into the [`NcCell`].
// ///
// /// *Method: NcCell.[load_char()][NcCell#method.load_char].*
// //
// // TODO:CHECK is this necessary at all?
// #[inline]
// pub fn nccell_load_char(plane: &mut NcPlane, cell: &mut NcCell, ch: char) /* -> i32 */
// {
//     let _ = unsafe { crate::nccell_load(plane, cell, ch) };
// }
// nccell_load_char(struct ncplane* n, nccell* c, char ch){
//   char gcluster[2];
//   gcluster[0] = ch;
//   gcluster[1] = '\0';
//   let _ = nccell_load(n, c, gcluster);
// }

// /// Loads a UTF-8 grapheme cluster of up to 4 bytes into the cell `c`.
// ///
// /// *Method: NcCell.[load_egc32()][NcCell#method.load_egc32].*
// //
// // TODO
// #[inline]
// pub fn nccell_load_egc32(plane: &mut NcPlane, cell: &mut NcCell, egc: &str) -> NcIntResult {
//     char gcluster[sizeof(egc) + 1];
//     egc = egc.to_le();
//     memcpy(gcluster, &egc, sizeof(egc));
//     gcluster[4] = '\0';
//     return nccell_load(n, c, gcluster);
// }
// // Load a UTF-8 encoded EGC of up to 4 bytes into the nccell 'c'. Returns the
// // number of bytes used, or -1 on error.
// static inline int
// nccell_load_egc32(struct ncplane* n, nccell* c, uint32_t egc){
//   char gcluster[sizeof(egc) + 1];
//   egc = htole(egc);
//   memcpy(gcluster, &egc, sizeof(egc));
//   gcluster[4] = '\0';
//   return nccell_load(n, c, gcluster);
// }

/// Copies the UTF8-encoded `EGC` out of the [`NcCell`], whether simple or complex.
///
/// The result is not tied to the [NcPlane],
/// and persists across erases and destruction.
///
/// *Method: NcCell.[strdup()][NcCell#method.strdup].*
#[inline]
pub fn nccell_strdup(plane: &NcPlane, cell: &NcCell) -> String {
    rstring![libc::strdup(crate::nccell_extended_gcluster(plane, cell))].into()
}

// Misc. -----------------------------------------------------------------------

/// Saves the [`NcStyle`] and the [`NcChannels`],
/// and returns the `EGC`, of an [`NcCell`].
///
/// *Method: NcCell.[extract()][NcCell#method.extract].*
#[inline]
pub fn nccell_extract(
    plane: &NcPlane,
    cell: &NcCell,
    stylemask: &mut NcStyle,
    channels: &mut NcChannels,
) -> String {
    if *stylemask != 0 {
        *stylemask = cell.stylemask;
    }
    if *channels != 0 {
        *channels = cell.channels;
    }
    nccell_strdup(plane, cell)
}

/// Returns true if the two cells are distinct `EGC`s, attributes, or channels.
///
/// The actual egcpool index needn't be the same--indeed, the planes needn't even
/// be the same. Only the expanded EGC must be equal. The EGC must be bit-equal;
///
/// *Method: NcCell.[compare()][NcCell#method.compare].*
//
// NOTE: FIXME: it would probably be better to test whether they're Unicode-equal
#[inline]
pub fn nccellcmp(plane1: &NcPlane, cell1: &NcCell, plane2: &NcPlane, cell2: &NcCell) -> bool {
    if cell1.stylemask != cell2.stylemask {
        return true;
    }
    if cell1.channels != cell2.channels {
        return true;
    }
    unsafe {
        strcmp(
            crate::nccell_extended_gcluster(plane1, cell1),
            crate::nccell_extended_gcluster(plane2, cell2),
        ) != 0
    }
}

/// Initializes (zeroes out) an [`NcCell`].
///
/// *Method: NcCell.[init()][NcCell#method.init].*
#[inline]
pub fn nccell_init(cell: &mut NcCell) {
    *cell = unsafe { core::mem::zeroed() }
}

/// Same as [`nccell_load`][crate::nccell_load], plus blasts the styling with
/// `style` and `channels`.
///
/// - Breaks the UTF-8 string in `gcluster` down, setting up the cell `cell`.
/// - Returns the number of bytes copied out of `gcluster`, or -1 on failure.
/// - The styling of the cell is left untouched, but any resources are released.
/// - Blasts the styling with `style` and `channels`.
///
/// *Method: NcCell.[prime()][NcCell#method.prime].*
pub fn nccell_prime(
    plane: &mut NcPlane,
    cell: &mut NcCell,
    gcluster: &str,
    style: NcStyle,
    channels: NcChannels,
) -> NcIntResult {
    cell.stylemask = style;
    cell.channels = channels;
    unsafe { crate::nccell_load(plane, cell, cstring![gcluster]) }
}

/// Loads up six cells with the `EGC`s necessary to draw a box.
///
/// Returns [`NCRESULT_OK`] on success or [`NCRESULT_ERR`] on error.
///
/// On error, any [`NcCell`]s this function might have loaded before the error
/// are [nccell_release]d. There must be at least six `EGC`s in `gcluster`.
///
/// *Method: NcCell.[load_box()][NcCell#method.load_box].*
pub fn nccells_load_box(
    plane: &mut NcPlane,
    style: NcStyle,
    channels: NcChannels,
    ul: &mut NcCell,
    ur: &mut NcCell,
    ll: &mut NcCell,
    lr: &mut NcCell,
    hl: &mut NcCell,
    vl: &mut NcCell,
    gcluster: &str,
) -> NcIntResult {
    assert![gcluster.len() >= 6]; // DEBUG

    // TODO: CHECK: mutable copy for pointer arithmetics:
    let mut gclu = cstring![gcluster];

    let mut ulen: NcIntResult;

    ulen = nccell_prime(plane, ul, gcluster, style, channels);

    if ulen > 0 {
        gclu = unsafe { gclu.offset(ulen as isize) };
        ulen = nccell_prime(plane, ur, gcluster, style, channels);

        if ulen > 0 {
            gclu = unsafe { gclu.offset(ulen as isize) };
            ulen = nccell_prime(plane, ll, gcluster, style, channels);

            if ulen > 0 {
                gclu = unsafe { gclu.offset(ulen as isize) };
                ulen = nccell_prime(plane, lr, gcluster, style, channels);

                if ulen > 0 {
                    gclu = unsafe { gclu.offset(ulen as isize) };
                    ulen = nccell_prime(plane, hl, gcluster, style, channels);

                    if ulen > 0 {
                        let _gclu = unsafe { gclu.offset(ulen as isize) };
                        ulen = nccell_prime(plane, vl, gcluster, style, channels);

                        if ulen > 0 {
                            return NCRESULT_OK;
                        }
                        unsafe {
                            nccell_release(plane, hl);
                        }
                    }
                    unsafe {
                        nccell_release(plane, lr);
                    }
                }
                unsafe {
                    nccell_release(plane, ll);
                }
            }
            unsafe {
                nccell_release(plane, ur);
            }
        }
        unsafe {
            nccell_release(plane, ul);
        }
    }
    NCRESULT_ERR
}
