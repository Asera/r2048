use crate::game_state::GameState;
use glium::{Display, Frame};
use nalgebra_glm as glm;
use glium_text_nxt::FontTexture;

pub fn render_state(state: &GameState, display: &Display, target: &mut Frame, font: &FontTexture) {
    let text_system = glium_text_nxt::TextSystem::new(display);

    for (x, row) in state.field.iter().enumerate() {
        for (y, element) in row.iter().enumerate() {
            let text = glium_text_nxt::TextDisplay::new(&text_system, font, element.power.to_string().as_str());
            let mut text_matrix = glm::translate(&glm::identity(), &glm::vec3(0.4 * x as f32 - 0.6, -0.4 * y as f32 + 0.6, 0.0));
            text_matrix = glm::scale(&text_matrix, &glm::vec3(0.1, 0.1, 0.1));
            glium_text_nxt::draw(&text, &text_system, target, text_matrix, (0.0, 0.0, 0.0, 1.0));
        }
    }

    let mut score_matrix = glm::translate(&glm::identity(), &glm::vec3(- 0.9, 0.9, 0.0));
    score_matrix = glm::scale(&score_matrix, &glm::vec3(0.05, 0.05, 0.05));

    let score_text = format!("Score: {}", state.score_current);
    let score_text_display = glium_text_nxt::TextDisplay::new(&text_system, font, score_text.as_str());
    glium_text_nxt::draw(&score_text_display, &text_system, target, score_matrix, (0.0, 0.0, 0.0, 1.0));

    let mut best_matrix = glm::translate(&glm::identity(), &glm::vec3(- 0.9, 0.8, 0.0));
    best_matrix = glm::scale(&best_matrix, &glm::vec3(0.05, 0.05, 0.05));

    let best_text = format!("Best: {}", state.score_best);
    let best_text_display = glium_text_nxt::TextDisplay::new(&text_system, font, best_text.as_str());
    glium_text_nxt::draw(&best_text_display, &text_system, target, best_matrix, (0.0, 0.0, 0.0, 1.0));
}