extern crate urlencoded;

use std::collections::HashMap;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data: &HashMap<String, Vec<String>> = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Błąd parsowania danych formularza: {:?}\n", e));
            return Ok(response);
        }
        Ok(data) => data,
    };

    let unparsed_numbers: &Vec<String> = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Formularz nie zawiera parametru 'n'\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Wartość parametru 'n' nie jest liczbą: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!(
        "Największy wspólny dzielnik {:?} to <b> {}</b> \n",
        numbers, d
    ));

    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n - m;
            n = t;
        }
        m = m % n;
    }
    n
}

extern crate iron;
extern crate router;
#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serwer dostępny pod adresem http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        r#"
        <title>Kalkualtor GCD</title>
        <form action="/gcd" method="post">
            <input type= "text" name="n"/>
            <input type= "text" name="n"/>
            <button type="submit"> Oblicz GCD </button>
        </form>
            "#,
    );

    Ok(response)
}
