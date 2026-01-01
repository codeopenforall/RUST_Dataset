use std::rc::Rc;
    data: Rc<Data>,
        self.data.value
    let data_rc = Rc::new(Data { value: 42 });
    let handler = Handler { data: Rc::clone(&data_rc) };
