void do_with_Foo(App* app, Foo obj);

         template<>
         void App::do_with<Foo>(Foo obj) {
             do_with_Foo(this, event);
         }
