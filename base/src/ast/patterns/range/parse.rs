use crate::{
    ast::patterns::{
        ObsoleteRangePattern, RangeExclusivePattern, RangeFromPattern, RangeInclusivePattern,
        RangePattern,
    },
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for RangePattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(lower) = parser.step_parse() {
            if let Ok(dots) = parser.step_parse() {
                return Ok(RangePattern::Obsolete(ObsoleteRangePattern {
                    lower,
                    dots,
                    upper: parser.parse()?,
                }));
            }

            if let Ok(dots) = parser.step_parse() {
                return Ok(RangePattern::Inclusive(RangeInclusivePattern {
                    lower,
                    dots,
                    upper: parser.parse()?,
                }));
            }

            let dots = parser.parse()?;

            if let Ok(upper) = parser.step_parse() {
                return Ok(RangePattern::Exclusive(RangeExclusivePattern {
                    lower,
                    dots,
                    upper,
                }));
            }

            return Ok(RangePattern::From(RangeFromPattern { lower, dots }));
        }

        if let Ok(to_inclusive) = parser.step_parse() {
            return Ok(RangePattern::ToInclusive(to_inclusive));
        }

        Err(parser.error("expected a range pattern"))
    }
}
