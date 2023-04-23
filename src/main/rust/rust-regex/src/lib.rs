use jni::{
    objects::{JClass, JObjectArray, JString},
    sys::jint,
    JNIEnv,
};
use regex::Regex;

#[no_mangle]
pub extern "system" fn Java_com_humio_jitrex_PerformanceTest_00024RustRegexBackend_internalCountMatches<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    pattern: JString<'local>,
    inputs: JObjectArray<'local>,
) -> jint {
    let pattern: String = env
        .get_string(&pattern)
        .expect("Couldn't get JNI string")
        .into();
    let regex = Regex::new(&pattern).expect("Failed to construct regex");

    let inputs_length = env
        .get_array_length(&inputs)
        .expect("failed to get array length");

    let mut matches = 0;

    for i in 0..inputs_length {
        let input = env
            .get_object_array_element(&inputs, i)
            .expect("Failed to get object array element");
        let input = JString::from(input);
        let input: String = unsafe {
            env.get_string_unchecked(&input)
                .expect("failed to get string")
                .into()
        };

        if regex.is_match(&input) {
            matches += 1;
        }
    }

    matches
}
