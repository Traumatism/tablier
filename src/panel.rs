#![allow(dead_code)]

pub struct PanelBox {
    horizontal: char,
    vertical: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
}

impl PanelBox {
    pub fn new(
        horizontal: char,
        vertical: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> PanelBox {
        PanelBox {
            horizontal: horizontal,
            vertical: vertical,
            top_left: top_left,
            top_right: top_right,
            bottom_left: bottom_left,
            bottom_right: bottom_right,
        }
    }

    pub fn ascii() -> PanelBox {
        PanelBox {
            horizontal: '-',
            vertical: '|',
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
        }
    }
}

pub struct Panel {
    pub content: &'static str,
    pub panel_box: PanelBox,
}

impl Panel {
    pub fn render(&self) -> String {
        if !&self.content.contains("\n") {
            let line = self
                .panel_box
                .horizontal
                .to_string()
                .repeat(self.content.len() + 2);

            let line_t = format!(
                "{}{}{}",
                self.panel_box.top_left, line, self.panel_box.top_right,
            );

            let line_b = format!(
                "{}{}{}",
                self.panel_box.bottom_left, line, self.panel_box.bottom_right,
            );

            return format!(
                "{}\n{} {} {}\n{}",
                line_t, self.panel_box.vertical, self.content, self.panel_box.vertical, line_b
            );
        }

        let lines = self.content.split('\n').collect::<Vec<&str>>();

        let max_l = lines.iter().map(|l| l.len()).max().unwrap();
        let line = self.panel_box.horizontal.to_string().repeat(max_l + 2);

        let mut output = String::new();

        output.push_str(&format!(
            "{}{}{}\n",
            self.panel_box.top_left, line, self.panel_box.top_right,
        ));

        for l in lines {
            output.push_str(&format!(
                "{} {}{} {}\n",
                self.panel_box.vertical,
                l,
                " ".repeat(max_l - l.len()),
                self.panel_box.vertical
            ))
        }

        output.push_str(&format!(
            "{}{}{}\n",
            self.panel_box.bottom_left, line, self.panel_box.bottom_right,
        ));

        output
    }
}
