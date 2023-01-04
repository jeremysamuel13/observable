use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Event<'a, EvtType, T> {
    pub event_name: String,
    pub event: Option<&'a EvtType>,
    pub src: T,
}

impl<'a, EvtType, T> Event<'a, EvtType, T> {
    pub fn new(src: T, event_name: impl ToString, event: Option<&'a EvtType>) -> Self {
        Self {
            event_name: event_name.to_string(),
            event,
            src,
        }
    }
}

#[derive(Clone)]
pub enum CallbackType<Callback, MutCallback> {
    Immutable(Callback),
    Mutable(MutCallback),
}

pub trait Observable<Evt, Callback = fn(Event<Evt, &Self>), MutCallback = fn(Event<Evt, &mut Self>)>
where
    Callback: Fn(Event<Evt, &Self>),
    MutCallback: Fn(Event<Evt, &mut Self>),
    Evt: ToString + Clone,
{
    //base functionality (api to inner map)

    fn get_callback(
        &self,
        event_name: impl ToString,
    ) -> Option<CallbackType<Callback, MutCallback>>;
    fn push_callback(&mut self, event: impl ToString, cb: CallbackType<Callback, MutCallback>);
    fn remove_callback(&mut self, event: impl ToString);
}

//seperate from Observable
pub trait ObservableInterface<
    Evt = String,
    Callback = fn(Event<Evt, &Self>),
    MutCallback = fn(Event<Evt, &mut Self>),
> where
    Callback: Fn(Event<Evt, &Self>),
    MutCallback: Fn(Event<Evt, &mut Self>),
    Evt: ToString + Clone,
    Self: Observable<Evt, Callback, MutCallback>,
{
    fn on(&mut self, event: impl ToString, f: Callback) {
        self.push_callback(event, CallbackType::Immutable(f))
    }
    fn off(&mut self, event: impl ToString) {
        self.remove_callback(event)
    }

    fn on_mut(&mut self, event: impl ToString, f: MutCallback) {
        self.push_callback(event, CallbackType::Mutable(f))
    }
    fn off_mut(&mut self, event: impl ToString) {
        self.remove_callback(event)
    }

    fn dispatch(&mut self, event_name: impl ToString, evt: Option<Evt>) -> bool {
        let str = event_name.to_string();
        let cb = self.get_callback(str.clone());

        match cb {
            Some(CallbackType::Immutable(im)) => {
                im(Event::new(self, &str, evt.as_ref()));
                true
            }
            Some(CallbackType::Mutable(mt)) => {
                mt(Event::new(self, &str, evt.as_ref()));
                true
            }
            _ => false, //no function found
        }
    }
}

impl<Evt, Callback, MutCallback, T: Observable<Evt, Callback, MutCallback>>
    ObservableInterface<Evt, Callback, MutCallback> for T
where
    Callback: Fn(Event<Evt, &Self>),
    MutCallback: Fn(Event<Evt, &mut Self>),
    Evt: ToString + Clone,
{
}

pub struct Return<Evt, Ret> {
    pub evt: Evt,
    pub ret: Ret,
}

impl<Evt, Ret> Return<Evt, Ret> {
    pub fn new(evt: Evt, ret: Ret) -> Self {
        Self { evt, ret }
    }
}
