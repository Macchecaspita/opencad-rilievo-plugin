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
        tr!("Rilievo", "Survey", "Levantamiento", "Relevé", "Ölçüm", "Levantamento", "Pomiar")
    }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![
                RibbonGroup {
                    title: tr!("Edificio", "Building", "Edificio", "Bâtiment", "Bina", "Edifício", "Budynek"),
                    tools: vec![
                        cmd("RV_PERIMETRO", tr!("1. Perimetro esterno", "1. Outer perimeter", "1. Perímetro exterior", "1. Périmètre extérieur", "1. Dış çevre", "1. Perímetro exterior", "1. Obrys zewnętrzny"), include_bytes!("../icons/perimeter.svg")),
                        cmd("RV_SPESSORI", tr!("2. Spessori muri esterni", "2. Outer wall thicknesses", "2. Espesores de muros exteriores", "2. Épaisseurs des murs extérieurs", "2. Dış duvar kalınlıkları", "2. Espessuras das paredes exteriores", "2. Grubości ścian zewnętrznych"), include_bytes!("../icons/walls.svg")),
                    ],
                },
                RibbonGroup {
                    title: tr!("Vani", "Rooms", "Estancias", "Pièces", "Odalar", "Compartimentos", "Pomieszczenia"),
                    tools: vec![
                        cmd("RV_VANO", tr!("3. Vani: numeri, lettere e misure", "3. Rooms: numbers, letters and measures", "3. Estancias: números, letras y medidas", "3. Pièces : numéros, lettres et mesures", "3. Odalar: numaralar, harfler ve ölçüler", "3. Compartimentos: números, letras e medidas", "3. Pomieszczenia: numery, litery i pomiary"), include_bytes!("../icons/room.svg")),
                        cmd("RV_LIBRETTO", tr!("Libretto misure", "Measures book", "Libro de medidas", "Carnet de mesures", "Ölçü defteri", "Caderno de medidas", "Dziennik pomiarów"), include_bytes!("../icons/book.svg")),
                    ],
                },
                RibbonGroup {
                    title: tr!("Aiuto", "Help", "Ayuda", "Aide", "Yardım", "Ajuda", "Pomoc"),
                    tools: vec![
                        cmd("RV_GUIDE", tr!("Guida", "Guide", "Guía", "Guide", "Rehber", "Guia", "Przewodnik"), include_bytes!("../icons/guide.svg")),
                        cmd("RV_MANUALE", tr!("Manuale PDF", "PDF manual", "Manual PDF", "Manuel PDF", "PDF kılavuz", "Manual PDF", "Podręcznik PDF"), include_bytes!("../icons/manual.svg")),
                    ],
                },
            ]
        })
    }
}
