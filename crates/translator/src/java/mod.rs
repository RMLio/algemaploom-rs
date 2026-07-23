use crate::api::process_one_str;
use jni::objects::{JClass, JString};
use jni::EnvUnowned;

#[unsafe(no_mangle)]
pub extern "system" fn Java_be_ugent_idlab_knows_mappingLoom_Translator_translate<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let outcome = unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
        let input_str = input.to_string();
        let translated_res = process_one_str(&input_str)
            .unwrap_or_else(|| "Error while translating the mapping".to_string());
        JString::from_str(env, translated_res.as_str())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
