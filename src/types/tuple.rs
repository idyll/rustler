use ::{ NifEnv, NifTerm, NifError, NifResult, NifEncoder, NifDecoder };
use ::wrapper::tuple;
use ::wrapper::nif_interface::NIF_TERM;

/// Convert an Erlang tuple to a Rust vector. (To convert to a Rust tuple, use `term.decode()`
/// instead.)
///
/// # Errors
/// `badarg` if `term` is not a tuple.
pub fn get_tuple<'a>(term: NifTerm<'a>) -> Result<Vec<NifTerm<'a>>, NifError> {
    let env = term.get_env();
    match unsafe { tuple::get_tuple(env.as_c_arg(), term.as_c_arg()) } {
        Ok(terms) => Ok(terms.iter().map(|x| NifTerm::new(env, *x)).collect::<Vec<NifTerm>>()),
        Err(_error) => Err(NifError::BadArg)
    }
}

/// Convert a vector of terms to an Erlang tuple. (To convert from a Rust tuple to an Erlang tuple,
/// use `NifEncoder` instead.)
pub fn make_tuple<'a>(env: NifEnv<'a>, terms: &[NifTerm]) -> NifTerm<'a> {
    let c_terms: Vec<NIF_TERM> = terms.iter().map(|term| term.as_c_arg()).collect();
    NifTerm::new(env, unsafe { tuple::make_tuple(env.as_c_arg(), &c_terms) })
}

/// Helper macro to emit tuple-like syntax. Wraps its arguments in parentheses, and adds a comma if
/// there's exactly one argument.
macro_rules! tuple {
    ( ) => { () };
    ( $e0:tt ) => { ($e0,) };
    ( $( $e:tt ),* ) => { ( $( $e ),* ) };
}

/// Helper macro that returns the number of comma-separated expressions passed to it.
/// For example, `count!(a + b, c)` evaluates to `2`.
macro_rules! count {
    ( ) => ( 0 );
    ( $blah:expr ) => ( 1 );
    ( $blah:expr, $( $others:expr ),* ) => ( 1 + count!( $( $others ),* ) )
}

macro_rules! impl_nifencoder_nifdecoder_for_tuple {
    ( $($index:tt : $tyvar:ident),* ) => {
        // No need for `$crate` gunk in here, since the macro is not exported.
        impl<$( $tyvar: NifEncoder ),*>
            NifEncoder for tuple!( $( $tyvar ),* )
        {
            fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
                let arr = [ $( NifEncoder::encode(&self.$index, env).as_c_arg() ),* ];
                NifTerm::new(env, unsafe {
                    tuple::make_tuple(env.as_c_arg(), &arr)
                })
            }
        }

        impl<'a, $( $tyvar: NifDecoder<'a> ),*>
            NifDecoder<'a> for tuple!( $( $tyvar ),* )
        {
            fn decode(term: NifTerm<'a>) -> NifResult<tuple!( $( $tyvar ),* )>
            {
                match unsafe { tuple::get_tuple(term.get_env().as_c_arg(), term.as_c_arg()) } {
                    Ok(elements) if elements.len() == count!( $( $index ),* ) =>
                        Ok(tuple!( $( (
                            <$tyvar as NifDecoder>::decode(
                                NifTerm::new(term.get_env(), elements[$index]))? ) ),* )),
                    _ =>
                        Err(NifError::BadArg),
                }
            }
        }
    }
}

impl_nifencoder_nifdecoder_for_tuple!();
impl_nifencoder_nifdecoder_for_tuple!(0: A);
impl_nifencoder_nifdecoder_for_tuple!(0: A, 1: B);
impl_nifencoder_nifdecoder_for_tuple!(0: A, 1: B, 2: C);
impl_nifencoder_nifdecoder_for_tuple!(0: A, 1: B, 2: C, 3: D);
impl_nifencoder_nifdecoder_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_nifencoder_nifdecoder_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_nifencoder_nifdecoder_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);