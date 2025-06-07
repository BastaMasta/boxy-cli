use boxy_cli::prelude::*;

fn main() {
    // Create a new Boxy instance with the macro
    let mut box1 = boxy!(type: BoxType::Double, color:"#00ffff");

    // Adding text segments with different colors
    box1.add_text_sgmt("Hello There Mateojablij trhwesoiuethj 0piswe hjgtgoise jgtowie3thj q3o-oitujpwiej toiq 0iweeh gt owjtpiewrwh WOKWRHJ JRQWE4IHYNE5R bfg oiwhf apeih aepih aepih aepihetm wf[ohgwlMRF [POWQWRF]] [OJTQEA [OJ]]OJBDGISUDBG SIUGRG OGUFOSIJGOSN SOGUIHSGIORNGR ORIRHGOSJRNGOIJRG OPIFGHRPGNPERIJG ORIRGRPIGNERPGOSJH ", "#ffff");
    box1.add_text_line("Hello Theree", "#32CD32");

    // Display the box
    box1.display();
    
}