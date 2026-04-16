# Form

```rust demo
let form_ref = ComponentRef::<FormRef>::default();
let on_submit = move |_| {
    // This is only called if all fields pass validation.
};

view! {
    <Form on_submit comp_ref=form_ref>
        <Field label="Username" name="username" required=true>
            <Input rules=vec![InputRule::required(true.into())]/>
        </Field>
        <Field label="Password" name="password" required=true>
            <Input input_type=InputType::Password rules=vec![InputRule::required(true.into())]/>
        </Field>
        <Button button_type=ButtonType::Submit>
            "Submit"
        </Button>
    </Form>
}
```

### Imperative Validation

Use `FormRef` to validate programmatically without submitting.

```rust demo
let form_ref = ComponentRef::<FormRef>::default();

view! {
    <Form comp_ref=form_ref>
        <Field label="Email" name="email" required=true>
            <Input rules=vec![InputRule::required(true.into())]/>
        </Field>
        <Button on_click=move |_| {
            let valid = form_ref.get_untracked().map_or(false, |f| f.validate());
            leptos::logging::log!("Form valid: {valid}");
        }>
            "Validate"
        </Button>
    </Form>
}
```

### Form Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` | |
| on_submit | `Option<BoxOneCallback<ev::SubmitEvent>>` | `None` | Called on form submit after validation passes. |
| comp_ref | `ComponentRef<FormRef>` | | Component ref for imperative validation. |
| children | `Children` | | |

### FormRef Methods

| Name | Parameters | Return | Description |
| --- | --- | --- | --- |
| validate | | `bool` | Validate all registered fields. Returns `true` if all pass. |
| validate_field | `name: String` | `bool` | Validate a specific field by name. |
