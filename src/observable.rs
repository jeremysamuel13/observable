use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Event<T> {
    pub event: String,
    pub src: T,
}

impl<T> Event<T> {
    pub fn new<E: ToString>(src: T, event: E) -> Self {
        Self {
            event: event.to_string(),
            src,
        }
    }
}

#[derive(Clone)]
pub enum CallbackType<Callback, MutCallback> {
    Immutable(Callback),
    Mutable(MutCallback),
}

pub trait Observable<Evt = String, Callback = fn(Event<&Self>), MutCallback = fn(Event<&mut Self>)>
where
    Callback: Fn(Event<&Self>),
    MutCallback: Fn(Event<&mut Self>),
    Evt: ToString,
{
    //base functionality (api to inner map)

    fn get_callback(&self, event: impl ToString) -> Option<CallbackType<Callback, MutCallback>>;
    fn push_callback(&mut self, event: impl ToString, cb: CallbackType<Callback, MutCallback>);
    fn remove_callback(&mut self, event: impl ToString);
}

//seperate from Observable
pub trait ObservableInterface<
    Evt = String,
    Callback = fn(Event<&Self>),
    MutCallback = fn(Event<&mut Self>),
> where
    Callback: Fn(Event<&Self>),
    MutCallback: Fn(Event<&mut Self>),
    Evt: ToString,
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

    fn dispatch(&mut self, event: impl ToString) -> bool {
        let str = event.to_string();

        let cb = self.get_callback(str.clone());

        match cb {
            Some(CallbackType::Immutable(im)) => {
                im(Event::new(self, str.clone()));
                true
            }
            Some(CallbackType::Mutable(mt)) => {
                mt(Event::new(self, str.clone()));
                true
            }
            _ => false, //no function found
        }
    }
}

impl<Evt, Callback, MutCallback, T: Observable<Evt, Callback, MutCallback>> ObservableInterface<Evt, Callback, MutCallback> for T
where
    Callback: Fn(Event<&Self>),
    MutCallback: Fn(Event<&mut Self>),
    Evt: ToString,
{
}
