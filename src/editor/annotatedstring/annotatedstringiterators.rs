use std::cmp::min;

use super::{Annotatedstring, AnnotatedStringPart};
pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_idx: usize,
}

impl <'a> AnnotatedStringIterator<'a>{
    type item = AnnotatedStringPart<'a>;

    fn next(&mut self) -> Option<Self::item>{
        if self.current_idx >= self.annotated_string.string.len(){
            return None;
        }

        if let Some(annotation) = self
            .annotated_string
            .annotations
            .iter()
            .filter(|annotations|{
                annotations.start_byte_idx <= self.current_idx
                && annotation.end_byte_idx > self.current_idx
            })
            .last()
            {
                let end_idx = min(annotation.end_byte_idx, self.annotated_string.string.len());
                let start_idx = self.current_idx;
                self.current_idx = end_idx;

                return Some(AnnotatedStringPart{
                    string: &self.annotated_string.string[start_idx..end_idx],
                    annotation_type: Some(annotation.annotation_type),
                });
            }

            // Find the boundary of the nearest annotation
            let mut end_idx = self.annotated_string.string.len();
            for annotation in &self.annotated_string.annotations {
                if annotation.start_byte_idx > self.current_idx && annotation.start_byte_idx < end_idx {
                    end_idx = annotation.start_byte_idx;
                }
            }
            let start_idx = self.current_idx;
            self.current_idx = end_idx;

            Some(AnnotatedStringPart{
                string: &self.annotatated_string.string[start_idx..end_idx],
                annotation_type: None,
            })
    }
}