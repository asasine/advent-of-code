//! This crate provides a procedural macro to generate the module structure for my solutions to the Advent of Code.

use std::str::FromStr;

use proc_macro as pm;
use proc_macro2 as pm2;
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
            Ok(Part::One)
        } else if ident == "part2" {
            Ok(Part::Two)
        } else {
            Err(syn::Error::new(ident.span(), "expected part1 or part2"))
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

/// Generates a main function to an Advent of Code solution with input from stdin.
///
/// The implementation automatically calls functions `part1` and `part2` with the input read from stdin. It assumes
/// they have the signature `fn foo(input: &str) -> usize`.
///
/// ```
/// # mod solutions { pub fn read_stdin() -> String { String::new() } pub fn setup_tracing() { } }
/// # fn part1(input: &str) -> usize { 0 }
/// # fn part2(input: &str) -> usize { 0 }
/// aoc_macro::aoc_main!();
/// ```
#[proc_macro]
pub fn aoc_main(_input: pm::TokenStream) -> pm::TokenStream {
    quote! {
        fn main() {
            solutions::setup_tracing();

            let mut start = ::std::time::Instant::now();
            let input = solutions::read_stdin();
            let reading = start.elapsed();

            start = ::std::time::Instant::now();
            println!("{}", part1(&input));
            let part1_time = start.elapsed();

            start = ::std::time::Instant::now();
            println!("{}", part2(&input));
            let part2_time = start.elapsed();

            ::tracing::debug!("Reading stdin: {:?}", reading);
            ::tracing::debug!("Part 1: {:?}", part1_time);
            ::tracing::debug!("Part 2: {:?}", part2_time);
        }
    }
    .into()
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
}
