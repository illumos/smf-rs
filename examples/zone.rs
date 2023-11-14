/*
 * Copyright 2023 Oxide Computer Company
 */

use smf::Result;

fn fil(n: usize, c: char) -> String {
    let mut s = String::new();
    while s.len() < n {
        s.push(c);
    }
    s
}

fn ind(n: usize) -> String {
    fil(n * 4, ' ')
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 {
        eprintln!("ERROR: which zone do you want to list services for?");
        std::process::exit(1);
    }

    let scf = smf::Scf::new_for_zone(&args[0])?;
    let scope = scf.scope_local()?;

    let mut services = scope.services()?;
    while let Some(service) = services.next().transpose()? {
        let n = service.name()?;

        println!("{}", fil(78, '='));
        println!("{}service: {}", ind(0), n);

        let mut instances = service.instances()?;
        while let Some(instance) = instances.next().transpose()? {
            println!("{}instance: {}", ind(1), instance.name()?);
        }

        println!();
    }

    Ok(())
}
