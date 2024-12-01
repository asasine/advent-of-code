//! This crate provides a procedural macro to generate the module structure for my solutions to the Advent of Code.

use std::str::FromStr;

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Token;

fn parse_prefixed_int<T>(input: ParseStream, prefix: &str) -> syn::Result<T>
where
    T: FromStr,
{
    let ident = input
        .parse::<syn::Ident>()
        .map_err(|e| syn::Error::new(e.span(), format!("expected {prefix}X")))?;

    let s = ident.to_string();
    if !s.starts_with(prefix) {
        return Err(syn::Error::new(
            ident.span(),
            format!("expected to start with {prefix}"),
        ));
    }

    let num = match s[prefix.len()..].parse::<T>() {
        Ok(num) => num,
        Err(_) => return Err(syn::Error::new(ident.span(), "expected a number")),
    };

    Ok(num)
}

#[derive(Debug, PartialEq, Eq)]
struct Year(u16);

impl Parse for Year {
    /// Parses a year from the input stream.
    ///
    /// A year follows the regex `r"year\d{4}"`.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_prefixed_int(input, "year").map(Year)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Day(u8);

impl Parse for Day {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_prefixed_int(input, "day").map(Day)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Part {
    One,
    Two,
}

impl Parse for Part {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        if ident == "part1" {
            return Ok(Part::One);
        } else if ident == "part2" {
            return Ok(Part::Two);
        } else {
            return Err(syn::Error::new(ident.span(), "expected part1 or part2"));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct YearDay {
    year: Year,
    day: Day,
}

impl Parse for YearDay {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year = input.parse::<Year>()?;

        input
            .parse::<Token![,]>()
            .map_err(|e| syn::Error::new(e.span(), "expected the day after the year"))?;

        let day = input.parse::<Day>()?;
        Ok(YearDay { year, day })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct YearDayPart {
    yd: YearDay,
    part: Part,
}

impl Parse for YearDayPart {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let yd = input.parse::<YearDay>()?;
        input
            .parse::<Token![,]>()
            .map_err(|e| syn::Error::new(e.span(), "expected the part after the year and day"))?;

        let part = input.parse::<Part>()?;
        Ok(YearDayPart { yd, part })
    }
}

struct Solution(syn::ItemFn);

impl Parse for Solution {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let func = syn::ItemFn::parse(input)?;
        Ok(Solution(func))
    }
}

impl ToTokens for Solution {
    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// Marks a function as an Advent of Code solution.
///
/// The first attribute is the year and the second attribute is the day. Both should be integers.
///
/// # Examples
/// ```
/// #[aoc_macro::aoc_solution(year2024, day1, part1)]
/// pub fn part1(input: &str) -> usize {
///    // --snip--
///    # 0
/// }
/// ```
#[proc_macro_attribute]
pub fn aoc_solution(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    aoc_solution_impl(attr.into(), item.into()).into()
}

fn aoc_solution_impl(attr: pm2::TokenStream, item: pm2::TokenStream) -> pm2::TokenStream {
    let _yd = match syn::parse2::<YearDayPart>(attr) {
        Ok(yd) => yd,
        Err(e) => return e.to_compile_error(),
    };

    let solution = match syn::parse2::<Solution>(item) {
        Ok(solution) => solution,
        Err(e) => return e.to_compile_error(),
    };

    solution.to_token_stream()
}

/// Generates a main function to run a specific Advent of Code solution.
///
/// ```
/// # mod solutions {
/// #     pub mod year2023 {
/// #         pub mod day01 {
/// #             pub fn part1(_s: &str) -> usize {
/// #                 0
/// #             }
/// #
/// #             pub fn part2(_s: &str) -> usize {
/// #                 0
/// #             }
/// #         }
/// #     }
/// # }
/// aoc_macro::aoc_main!(year2023, day1);
/// ```
#[proc_macro]
pub fn aoc_main(input: pm::TokenStream) -> pm::TokenStream {
    aoc_main_impl(input.into()).into()
}

fn aoc_main_impl(input: pm2::TokenStream) -> pm2::TokenStream {
    let yd = match syn::parse2::<YearDay>(input) {
        Ok(yd) => yd,
        Err(e) => return e.to_compile_error(),
    };

    let year_prefixed = format!("year{}", yd.year.0);
    let day_prefixed = format!("day{:02}", yd.day.0);
    let include_str_path = format!("year{}/inputs/{:02}.txt", yd.year.0, yd.day.0);

    fn make_path_expr(segments: &[&str]) -> syn::ExprPath {
        syn::ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: segments
                    .iter()
                    .map(|s| syn::PathSegment {
                        ident: format_ident!("{}", s),
                        arguments: syn::PathArguments::None,
                    })
                    .collect(),
            },
        }
    }

    let part1_expr = make_path_expr(&["solutions", &year_prefixed, &day_prefixed, "part1"]);
    let part2_expr = make_path_expr(&["solutions", &year_prefixed, &day_prefixed, "part2"]);

    quote! {
        fn main() {
            let input = include_str!(#include_str_path);
            println!("{}", #part1_expr(input));
            println!("{}", #part2_expr(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_year() {
        let input = quote! {
            year2024
        };

        let actual = syn::parse2::<Year>(input).unwrap();
        assert_eq!(Year(2024), actual);
    }

    #[test]
    fn parse_day() {
        let input = quote! {
            day01
        };

        let actual = syn::parse2::<Day>(input).unwrap();
        assert_eq!(Day(1), actual);
    }

    #[test]
    fn parse_year_day() {
        let input = quote! {
            year2024, day01
        };

        let actual = syn::parse2::<YearDay>(input).unwrap();
        let expected = YearDay {
            year: Year(2024),
            day: Day(1),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_part() {
        let input = quote! {
            part1
        };

        let actual = syn::parse2::<Part>(input).unwrap();
        assert_eq!(Part::One, actual);
    }

    #[test]
    fn parse_year_day_part() {
        let input = quote! {
            year2024, day01, part1
        };

        let actual = syn::parse2::<YearDayPart>(input).unwrap();
        let expected = YearDayPart {
            yd: YearDay {
                year: Year(2024),
                day: Day(1),
            },
            part: Part::One,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn aoc_main_macro() {
        let actual = aoc_main_impl(quote! {
            year2024, day01
        });

        let expected = quote! {
            fn main() {
                let input = include_str!("year2024/inputs/01.txt");
                println!("{}", solutions::year2024::day01::part1(input));
                println!("{}", solutions::year2024::day01::part2(input));
            }
        };

        assert_eq!(expected.to_string(), actual.to_string());
    }
}
