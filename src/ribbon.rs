//! The "Rilievo" ribbon tab. Pure data — the host renders it.

use ocs_plugin_api::ribbon::{CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef};

pub struct RilievoModule;

fn cmd(id: &'static str, label: &'static str, svg: &'static [u8]) -> RibbonItem {
    RibbonItem::LargeTool(ToolDef {
        id,
        label,
        icon: IconKind::Svg(svg),
        event: ModuleEvent::Command(id.to_string()),
    })
}

impl CadModule for RilievoModule {
    fn id(&self) -> &'static str {
        "rilievovani"
    }

    fn title(&self) -> &'static str {
        tr!("Rilievo", "Survey")
    }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![
                RibbonGroup {
                    title: tr!("Vano", "Room"),
                    tools: vec![
                        cmd("RV_ETICHETTE", tr!("Lettere vertici", "Corner letters"), include_bytes!("../icons/labels.svg")),
                        cmd("RV_VANO", tr!("Misure e diagonali", "Sides and diagonals"), include_bytes!("../icons/room.svg")),
                        cmd("RV_LIBRETTO", tr!("Libretto misure", "Measures book"), include_bytes!("../icons/book.svg")),
                    ],
                },
                RibbonGroup {
                    title: tr!("Aiuto", "Help"),
                    tools: vec![cmd("RV_GUIDE", tr!("Guida", "Guide"), include_bytes!("../icons/guide.svg"))],
                },
            ]
        })
    }
}
