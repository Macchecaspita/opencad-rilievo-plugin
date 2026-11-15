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
                    title: tr!("Vano", "Room", "Estancia", "Pièce", "Oda", "Compartimento", "Pomieszczenie"),
                    tools: vec![
                        cmd("RV_ETICHETTE", tr!("Lettere vertici", "Corner letters", "Letras de esquinas", "Lettres des angles", "Köşe harfleri", "Letras dos cantos", "Litery narożników"), include_bytes!("../icons/labels.svg")),
                        cmd("RV_VANO", tr!("Misure e diagonali", "Sides and diagonals", "Lados y diagonales", "Côtés et diagonales", "Kenarlar ve köşegenler", "Lados e diagonais", "Boki i przekątne"), include_bytes!("../icons/room.svg")),
                        cmd("RV_APPLICA", tr!("Applica al disegno", "Apply to drawing", "Aplicar al dibujo", "Appliquer au dessin", "Çizime uygula", "Aplicar ao desenho", "Zastosuj na rysunku"), include_bytes!("../icons/apply.svg")),
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
