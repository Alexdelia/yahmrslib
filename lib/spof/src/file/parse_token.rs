use super::SpofedFile;

use crate::{FoundLine, ParsedLine};

use std::num::ParseFloatError;
use std::path::Path;
use std::str::FromStr;

impl SpofedFile {
    // fn parse_vector
    // might code that in spof

    fn parse_line(
        obj_path: &Path,
        fl: &FoundLine,
    ) -> Result<Vec<Vec<VertexPrecision>>, (usize, (String, ParseFloatError))> {
        let mut v = Vec::new();

        for (i, pl) in fl.0.iter().enumerate() {
            v.push(parse_coord(pl).map_err(|e| (i, e))?);
        }

        Ok(v)
    }

    fn parse_coord(pl: &ParsedLine) -> Result<Vec<VertexPrecision>, (String, ParseFloatError)> {
        let mut v = Vec::new();

        for (i, t) in pl.0.iter().enumerate() {
            v.push(t.parse::<VertexPrecision>().map_err(|e| (t.clone(), e))?);
        }

        Ok(v)
    }
}
