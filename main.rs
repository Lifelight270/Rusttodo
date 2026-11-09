use leptos::{ev, prelude::*};

#[derive(Clone)]
struct Todo {
    id: u32,
    text: String,
    done: RwSignal<bool>,
}

impl Todo {
    fn new(id: u32, text: &str, done: bool) -> Self {
        Self { id, text: text.to_string(), done: RwSignal::new(done) }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Open,
    Done,
}

#[component]
fn App() -> impl IntoView {
    let todos = RwSignal::new(vec![
        Todo::new(1, "Ship the onboarding copy", false),
        Todo::new(2, "Book the dentist for Thursday", false),
        Todo::new(3, "Read twenty pages", true),
    ]);
    let next_id = RwSignal::new(4u32);
    let input = RwSignal::new(String::new());
    let filter = RwSignal::new(Filter::All);
    let pointer = RwSignal::new((-600.0_f64, -600.0_f64));

    // Cursor-follow spotlight
    window_event_listener(ev::pointermove, move |e| {
        pointer.set((e.client_x() as f64, e.client_y() as f64));
    });

    let total = move || todos.with(|l| l.len());
    let done_count = move || todos.with(|l| l.iter().filter(|t| t.done.get()).count());
    let pct = move || if total() == 0 { 0 } else { done_count() * 100 / total() };

    let visible = move || {
        let f = filter.get();
        todos
            .get()
            .into_iter()
            .filter(|t| match f {
                Filter::All => true,
                Filter::Open => !t.done.get(),
                Filter::Done => t.done.get(),
            })
            .collect::<Vec<_>>()
    };

    let add = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let text = input.get().trim().to_string();
        if text.is_empty() {
            return;
        }
        let id = next_id.get();
        next_id.set(id + 1);
        todos.update(|l| l.insert(0, Todo::new(id, &text, false)));
        input.set(String::new());
    };

    view! {
        <div
            class="spot"
            aria-hidden="true"
            style=move || {
                let (x, y) = pointer.get();
                format!("transform: translate3d({}px, {}px, 0)", x - 260.0, y - 260.0)
            }
        ></div>

        <main class="shell">
            <section class="panel">
                <header class="head">
                    <h1>"What are you getting done today?"</h1>
                    <div
                        class="ring"
                        role="img"
                        aria-label=move || format!("{} of {} tasks done", done_count(), total())
                    >
                        <svg viewBox="0 0 64 64">
                            <circle class="track" cx="32" cy="32" r="26"></circle>
                            <circle
                                class="bar"
                                cx="32"
                                cy="32"
                                r="26"
                                stroke-dasharray="163.36"
                                stroke-dashoffset=move || format!("{:.2}", 163.36 * (1.0 - pct() as f64 / 100.0))
                            ></circle>
                        </svg>
                        <span>{move || format!("{}%", pct())}</span>
                    </div>
                </header>

                <form class="add" on:submit=add>
                    <input
                        type="text"
                        placeholder="Add a task"
                        aria-label="New task"
                        prop:value=move || input.get()
                        on:input=move |e| input.set(event_target_value(&e))
                    />
                    <button type="submit" class="go">"Add task"</button>
                </form>

                <div
                    class="seg"
                    style=move || {
                        let i = match filter.get() {
                            Filter::All => 0,
                            Filter::Open => 1,
                            Filter::Done => 2,
                        };
                        format!("--i:{i}")
                    }
                >
                    <span class="thumb" aria-hidden="true"></span>
                    {[(Filter::All, "All"), (Filter::Open, "Open"), (Filter::Done, "Done")]
                        .into_iter()
                        .map(|(f, label)| {
                            view! {
                                <button
                                    type="button"
                                    class:on=move || filter.get() == f
                                    aria-pressed=move || (filter.get() == f).to_string()
                                    on:click=move |_| filter.set(f)
                                >
                                    {label}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                <ul class="list">
                    <For each=visible key=|t| t.id let:todo>
                        {
                            let done = todo.done;
                            let id = todo.id;
                            view! {
                                <li class="item" class:done=move || done.get()>
                                    <button
                                        type="button"
                                        class="check"
                                        role="checkbox"
                                        aria-checked=move || done.get().to_string()
                                        aria-label="Mark as done"
                                        on:click=move |_| done.update(|d| *d = !*d)
                                    >
                                        <svg viewBox="0 0 24 24"><path d="M5 12.5l4.5 4.5L19 7.5"></path></svg>
                                    </button>
                                    <span class="text">{todo.text}</span>
                                    <button
                                        type="button"
                                        class="del"
                                        aria-label="Delete task"
                                        on:click=move |_| todos.update(|l| l.retain(|t| t.id != id))
                                    >
                                        "×"
                                    </button>
                                </li>
                            }
                        }
                    </For>
                </ul>

                <Show when=move || visible().is_empty()>
                    <p class="empty">
                        {move || match filter.get() {
                            Filter::All => "No tasks yet. Add your first one above.",
                            Filter::Open => "Everything is done. Nice.",
                            Filter::Done => "Nothing finished yet.",
                        }}
                    </p>
                </Show>

                <footer class="foot">
                    <span>{move || format!("{} left", total() - done_count())}</span>
                    <button
                        type="button"
                        class="ghost"
                        disabled=move || done_count() == 0
                        on:click=move |_| todos.update(|l| l.retain(|t| !t.done.get_untracked()))
                    >
                        "Clear completed"
                    </button>
                </footer>
            </section>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
