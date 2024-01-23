use std::io;
fn main() {
    println!("\n\n\t Salut Séb !!! \n");
    println!("\n\n\t Programme DVT \n");
    println!(" \t Vous cherchez :");
    println!(" Une distance --- tapez: 1");
    println!(" Une vitesse  --- tapez: 2");
    println!(" Une durée    --- tapez: 3");
    println!(" \n\n\t Choix:");
    let mut choix = String::new();

    io::stdin()
        .read_line(&mut choix)
        .expect(" Echec de l'entrée utilisateur");

        let choix: u32 = choix.trim().parse().expect("Veuillez entrer un nombre !");

    println!("\n\t Vous avez choisi: {}\n\n\t", choix);

    if choix == 1 {
        println!("Vous chercher une distance");
        Fonction_Distance ();    
    }
    if choix == 2 {
        println!("Vous chercher une vitesse");
        Fonction_Vitesse ();
    }
    if choix == 3 {
        println!("Vous chercher une durée");
        Fonction_Durée ();
    } 
    if choix == 0 || choix > 3  {
        println!("Je ne sais pas ce que vous cherchez !!!");
    }            
}

// *************************************************************************************
fn Fonction_Vitesse ()
{
    println!("\n*** Fonction_Vitesse ***\n");   

    println! ("\nEntrez une distance:");
    let mut DistanceSaisie =String::new();
    io::stdin()
          .read_line(&mut DistanceSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
     
     let DistanceSaisie: f32  = DistanceSaisie.trim().parse().expect("Veuillez entrer un nombre !");
     println!("\nVotre distance: {}", DistanceSaisie);
 
    
    println! ("\nEntrez une durée:");
    let mut DuréeSaisie = String::new();
    io::stdin()
          .read_line(&mut DuréeSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
     
    let DuréeSaisie: f32 = DuréeSaisie.trim().parse().expect("Veuillez entrer un nombre !");
    println!("\nVotre durée: {}", DuréeSaisie); 
    
    let vitesse = (DistanceSaisie / DuréeSaisie) ;
    println!("\n La vitesse est de {}", vitesse);
}
    
// *************************************************************************************
fn Fonction_Durée ()
{
        println!("\n*** Fonction_Durée ***\n");

        println! ("\nEntrez une distance:");
        let mut DistanceSaisie =String::new();
        io::stdin()
              .read_line(&mut DistanceSaisie)
              .expect("Échec de la lecture de l'entrée utilisateur");
         
         let DistanceSaisie: f32  = DistanceSaisie.trim().parse().expect("Veuillez entrer un nombre !");
         println!("\nVotre distance: {}", DistanceSaisie);


         println! ("\nEntrez une vitesse:");
         let mut VitesseSaisie = String::new();
         io::stdin()
               .read_line(&mut VitesseSaisie)
               .expect("Échec de la lecture de l'entrée utilisateur");
          
         let VitesseSaisie: f32 = VitesseSaisie.trim().parse().expect("Veuillez entrer un nombre !");
         println!("\nVotre vitesse: {}", VitesseSaisie); 
        let Durée = DistanceSaisie / VitesseSaisie;
        println!("\n\t\t La durée est de {}",Durée);
}

// *************************************************************************************
fn Fonction_Distance ()
{
//    distance = Durée * vitesse;
    println!("\n*** Fonction_Distance ***\n");
    println! ("\nEntrez une durée:");
    let mut DuréeSaisie = String::new();
    io::stdin()
          .read_line(&mut DuréeSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
    let DuréeSaisie: f32 = DuréeSaisie.trim().parse().expect("Veuillez entrer un nombre !");
    println!("\nVotre durée: {}", DuréeSaisie); 

    println! ("\nEntrez une vitesse:");
    let mut VitesseSaisie = String::new();
    io::stdin()
          .read_line(&mut VitesseSaisie)
          .expect("Échec de la lecture de l'entrée utilisateur");
    let VitesseSaisie: f32 = VitesseSaisie.trim().parse().expect("Veuillez entrer un nombre !");
    println!("\nVotre vitesse: {}", VitesseSaisie); 

let distance =VitesseSaisie*DuréeSaisie;
println!("\n\t\t La distance est de {}", distance);
}
