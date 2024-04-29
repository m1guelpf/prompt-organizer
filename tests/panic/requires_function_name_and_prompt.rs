use prompt_organizer::prompt;

prompt! {requires_prompt}
prompt! {"This is only the prompt, no function name is provided"}
prompt! {}
prompt!();

fn main() {}
