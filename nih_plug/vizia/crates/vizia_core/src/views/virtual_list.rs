use crate::prelude::*;

#[derive(Lens)]
pub struct VirtualList {
    num_items: usize,
    item_height: f32,
    visible_items: Vec<usize>,
}

pub enum VirtualListEvent {
    SetNumItems(usize),
    SetScrollY(f32),
}

impl VirtualList {
    pub fn new<V: View, L, T>(
        cx: &mut Context,
        list: L,
        height: f32,
        item: impl Fn(&mut Context, usize, Then<L, Index<Vec<T>, T>>) -> Handle<V> + 'static,
    ) -> Handle<Self>
    where
        L: Lens<Target = Vec<T>>,
        T: Data + 'static,
    {
        let num_items = list.get(cx).len();

        Self { num_items, item_height: height, visible_items: (0..10).collect::<Vec<_>>() }.build(
            cx,
            |cx| {
                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                    VStack::new(cx, |cx| {
                        Binding::new(cx, VirtualList::visible_items, move |cx, visible_list| {
                            for i in visible_list.get(cx) {
                                let ptr = list.clone().index(i);
                                (item)(cx, i, ptr)
                                    .top(Pixels(i as f32 * height))
                                    .height(Pixels(height))
                                    .position_type(PositionType::SelfDirected);
                            }
                        });
                    })
                    .height(Pixels(num_items as f32 * height));
                    // List::new(cx, VirtualList::visible_items, move |cx, index, _|{
                    //     (item)(cx, index);
                    // })
                    // .height(Pixels(num_items as f32 * height));
                })
                .on_scroll(|cx, _, y| {
                    //println!("Scroll {} {}", x, y);
                    cx.emit(VirtualListEvent::SetScrollY(y));
                });
                // .on_geo_changed(move |cx, geo| {
                //     let current = cx.current();
                //     let dpi = cx.scale_factor();
                //     let container_height = cx.cache.get_height(current);
                //     let num_items = (container_height / height / dpi).ceil() as usize;
                //     cx.emit(VirtualListEvent::SetNumItems(num_items));
                //     //println!("Num Visible Items: {} {} {}", container_height, height, num_items);
                // });
            },
        )
        //.height(Pixels(num_items as f32 * height))
    }
}

impl View for VirtualList {
    fn element(&self) -> Option<&'static str> {
        Some("virtual_list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|virtual_list_event, _| match virtual_list_event {
            VirtualListEvent::SetNumItems(num_items) => {
                self.visible_items.clear();
                for i in 0..*num_items {
                    self.visible_items.push(i);
                }
            }

            VirtualListEvent::SetScrollY(scrolly) => {
                let current = cx.current();
                let dpi = cx.scale_factor();
                let container_height = cx.cache.get_height(current) / dpi;
                let total_height = self.num_items as f32 * self.item_height;
                let offsety = ((total_height - container_height) * *scrolly).round() * dpi;
                let mut offset_num = (offsety / self.item_height / dpi).ceil() as usize;
                offset_num = offset_num.saturating_sub(1);
                // let num_visible = self.visible_items.len();
                let num_items = (container_height / self.item_height).ceil() as usize;
                //println!("list: {} {}", offset_num, offset_num+num_visible);
                self.visible_items.clear();
                for i in offset_num..(offset_num + num_items) {
                    self.visible_items.push(i);
                }
            }
        });
    }
}
