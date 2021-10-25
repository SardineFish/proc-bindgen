template <>
void App::push_event<Foo>(Foo event)
{
    __app_push_event_Foo(this, event);
}
