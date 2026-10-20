use crate::animation::engine::DataStore;
use crate::animation::functions::{fuck_up_text, split_word_template, work_out_date};

pub fn broadcasts() -> DataStore {
    let mut broadcasts: DataStore = DataStore::new();

    let ocean_b_0: String = concat!(
        "Now for the official national~ weather~ service~ forecast\n",
        "~~~~for~~ Eastern Massachusetts~ inside of~~ I-~~~~4~~~~9~~~~~~~5~~,\n",
        "~~~~~~~~    including Boston,\n",
        "\n",
        "~~~~~~~~issued at 7~~~~:2~~1~~~~ PM~~~~, ~~~~~~~~~~~~~~Thursday, October~~ 2~~2~~nd.",
    ).to_string();
    broadcasts.set::<String>("ocean_b_0", ocean_b_0);

    let ocean_b_1: String = concat!(
        "Tonight:\n",
        "\n",
        "~~~~~Mostly cloudy with isolated~ showers~ until~~ mid~~~~night,\n",
        "~~~~~~~~~then mostly clear~~ after~~ mid~night.\n",
        "~~~~~~~~~Lows in the lower 4~~~~0~~~~s.\n",
        "~~~~~~~~~West winds 10~ to~~ 1~~5~~ miles~~ an~~ hour\n",
        "~~~~~with~ gusts~~ up~ to~ 2~~~~5~~~~ miles~~ an~~ hour~~.\n",
        "~~~~~~~~~~Chance of rain:~~~~ 2~~0~~~~ per~cent.",
    ).to_string();
    broadcasts.set::<String>("ocean_b_1", ocean_b_1);

    let ocean_b_2: String = concat!(
        "Friday:\n",
        "\n",
        "~~Sunny.\n",
        "~~~~~~~~~~~~~~~~Lush colour with highs in the low~er 5~~~0~s.\n",
        "~~~~~~~~~~Northwest~~ winds~~ 10~~-~1~~5~~ miles~~ an~~ hour\n",
        "~~~~with gusts up to~~ 2~~~~5~~~~ miles~~ an~~ hour~~.\n",
        "~~~~~~~~~~~~~~~~Friday night,~~ mostly~ clear.\n",
        "~~~~~~~~~~~~~~Lows in the mid-3~~~~0~~s.\n",
        "~~~~~~~~~~~~~~~~North winds 10-1~~~5~~ miles~~ an~~ hour~~.",
    ).to_string();
    broadcasts.set::<String>("ocean_b_2", ocean_b_2);

    let ocean_b_3: String = concat!(
        "Saturday:\n",
        "\n",
        "~~~~~~~~Partly sunny.\n",
        "~~~~~~~~High-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-@igh-i",
    ).to_string();
    broadcasts.set::<String>("ocean_b_3", ocean_b_3);

    let ocean_c_0: String = concat!(
        "Here~ are~ the 7~~~~ P~~~~M~~~~ ob~ser~va~tions for~~ the\n",
        "~~~Bos~ton~~ metro~po~li~tan~~~ ar~ea.\n",
        "\n",
        "~~~~~~~~~~~~~~~At Logan~~~~ Airport,~~~~~~ it was clou~~~~dy.\n",
        "~~~~~~~~~~~~~~~The tem~per~a~ture was 6~~~~8~~~~ de~grees,\n",
        "~~~~~~~~the dew point,~~ 4~~~~7~~ -\n",
        "~~~~~~and~ the~ re~la~tive~~ hu~mi~di~ty,~~~~ 4~~~~6~~~~ per~~cent.\n",
        "~~~~~~~~~~~~~~~~The~ wind~ was~ south~~west~ at 1~~~~3~ miles~~~~ an~~~~ ho~ur.\n",
        "~~~~~~~~~~~~The~ pres~~sure~~ was~~ 2~~~~9~~~~.~~~~~~~~9~~~~~~9~~~~~~ in~ches and ri~sing.\n",
        "~~~~~~~~Elsewher",
    ).to_string()
        + &fuck_up_text("rrrrrrrrr\n", 800.0, "");
    broadcasts.set::<String>("ocean_c_0", ocean_c_0);

    let ocean_c_1: String = fuck_up_text(concat!(
        "Here~ are~ the 7~~~~ P~~~~M~~~~ ob~ser~va~tions for~~ the\n",
        "~~~Bos~ton~~ metro~po~li~tan~~~ ar~ea.\n",
        "\n",
        "~~~~~~~~~~~~~~~At Logan~~~~ Airport,~~~~~~ it was clou~~~~dy.\n",
        "~~~~~~~~~~~~~~~The tem~per~a~ture was 6~~~~8~~~~ de~grees,\n",
        "~~~~~~~~the dew point,~~ 4~~~~7~~ -\n",
        "~~~~~~and~ the~ re~la~tive~~ hu~mi~di~ty,~~~~ 4~~~~6~~~~ per~~cent.\n",
        "~~~~~~~~~~~~~~~~The~ wind~ was~ south~~west~ at 1~~~~3~ miles~~~~ an~~~~ ho~ur.\n",
        "~~~~~~~~~~~~The~ pres~~sure~~ was~~ 2~~~~9~~~~.~~~~~~~~9~~~~~~9~~~~~~ in~ches and ri~sing.\n",
        "~~~~~~~~Elsewher",
    ), 100.0, "")
        + &fuck_up_text("rrrrrrrrr\n", 900.0, "");
    broadcasts.set::<String>("ocean_c_1", ocean_c_1);

    let ocean_c_2: String = fuck_up_text(concat!(
        "Here~ are~ the 7~~~~ P~~~~M~~~~ ob~ser~va~tions for~~ the\n",
        "~~~Bos~ton~~ metro~po~li~tan~~~ ar~ea.\n",
        "\n",
        "~~~~~~~~~~~~~~~At Logan~~~~ Airport,~~~~~~ it was clou~~~~dy.\n",
        "~~~~~~~~~~~~~~~The tem~per~a~ture was 6~~~~8~~~~ de~grees,\n",
        "~~~~~~~~the dew point,~~ 4~~~~7~~ -\n",
        "~~~~~~and~ the~ re~la~tive~~ hu~mi~di~ty,~~~~ 4~~~~6~~~~ per~~cent.\n",
        "~~~~~~~~~~~~~~~~The~ wind~ was~ south~~west~ at 1~~~~3~ miles~~~~ an~~~~ ho~ur.\n",
        "~~~~~~~~~~~~The~ pres~~sure~~ was~~ 2~~~~9~~~~.~~~~~~~~9~~~~~~9~~~~~~ in~ches and ri~sing.\n",
        "~~~~~~~~Elsewher",
    ), 200.0, "")
        + &fuck_up_text("rrrrrrrrr\n", 1000.0, "");
    broadcasts.set::<String>("ocean_c_2", ocean_c_2);

    let ocean_c_3: String = fuck_up_text(concat!(
        "Here~ are~ the 7~~~~ P~~~~M~~~~ ob~ser~va~tions for~~ the\n",
        "~~~Bos~ton~~ metro~po~li~tan~~~ ar~ea.\n",
        "\n",
        "~~~~~~~~~~~~~~~At Logan~~~~ Airport,~~~~~~ it was clou~~~~dy.\n",
        "~~~~~~~~~~~~~~~The tem~per~a~ture was 6~~~~8~~~~ de~grees,\n",
        "~~~~~~~~the dew point,~~ 4~~~~7~~ -\n",
        "~~~~~~and~ the~ re~la~tive~~ hu~mi~di~ty,~~~~ 4~~~~6~~~~ per~~cent.\n",
        "~~~~~~~~~~~~~~~~The~ wind~ was~ south~~west~ at 1~~~~3~ miles~~~~ an~~~~ ho~ur.\n",
        "~~~~~~~~~~~~The~ pres~~sure~~ was~~ 2~~~~9~~~~.~~~~~~~~9~~~~~~9~~~~~~ in~ches and ri~sing.\n",
        "~~~~~~~~Elsewher",
    ), 250.0, "")
        + &fuck_up_text("rrrrrrrrr\n", 1000.0, "");
    broadcasts.set::<String>("ocean_c_3", ocean_c_3);

    let funding_0: String = concat!(
        "Fun####ding#### for#### this#### pro####gram#### was#### made#### pos####si####ble###~\n",
        "  by###\n",
        "    by###\n",
        "      by###\n",
        "        by###\n",
        "          by#\n",
        "Fun#\n",
        "   by\n",
        "     by\n",
        "       by\n",
        "         by\n",
        "Funding#\n",
        "       by\n",
        "         by\n",
        "           by\n",
        "             by\n",
        "Funding for\n",
        "Funding for thi#i#i#i#i#\n",
        "Funding for this pro####gram###\n",
        "Funding for this pro####gram###\n",
        "                   pro\n",
        "                     pro\n",
        "                       pro\n",
        "Funding for this pro#gram.#~\n",
        "Fun####ding#### for#\n",
        "           by#\n",
        "             by#\n",
        "Funding made#### pos####si####ble#### by#### view####ers#### like#### you.###~\n",
        "####like#### you.##\n",
        "####like#### you.##\n",
        "####like#### you.##\n",
        "####like#### you.##\n",
        "####like#### you.##\n",
        "####like#### you.\n",
        " Fu\n",
        "  Fu\n",
    ).to_string();
    let mut funding_0: Vec<Vec<String>> = split_word_template(&funding_0);
    funding_0.extend_from_within(..);
    broadcasts.set::<Vec<Vec<String>>>("funding_0", funding_0);

    let funding_1: String = concat!(
        "Broad####cast####\n",
        "Broadcast Cor####por##a####tion.~#####\n",
        "Cor####po##ra####tion.#####\n",
        "Cor####po##ra####tion.#####\n",
        " Cor###po\n",
        "  Cor###po\n",
        "    Co\n",
        "      Co\n",
        "Cor####po##ra####tion.#####\n",
        "Cor####po##ra####tion.#####\n",
        "Cor####po##ra####tion.#####\n",
        " Cor###po\n",
        "  Cor###po\n",
        "    Co\n",
        "      Co\n",
        "Cor####po##ra###tion.#\n",
        " Co\n",
        "  Co\n",
        "Cor####po##ra###tion.#\n",
        " Co\n",
        "  Co\n",
        "Cor####po##ra###tion.#\n",
        " Co\n",
        "  Co\n",
        "Cor####po##ra###tion.#\n",
        " Co\n",
        "  Co\n",
        "Cor####po##ra###tion.#\n",
        " Co\n",
        "  Co\n",
        "Cor####po##ra###tion.#\n",
        " Co\n",
        "  Co\n",
        " Cor###po\n",
        "  Cor###po\n",
        "    Cor###po\n",
        "      Cor###po\n",
        "...#######\n",
        " Cor###po#\n",
        "     cor##\n",
        "  cor###po#\n",
        "    cor###po#\n",
        " cor##\n",
        "      cor###po#\n",
        "  cor###po#\n",
        "         cor##\n",
        "     cor###po#\n",
        " cor##\n",
        "   cor##\n",
        "     cor##\n",
        "       co#\n",
        "     co#\n",
        " cor###po#\n",
        "   cor##\n",
        " cor###po#\n",
        "   cor###po#\n",
        "     cor##\n",
        "       cor##\n",
        "         cor###po#\n",
    ).to_string()
        + &fuck_up_text(concat!(
        " cor###po#\n",
        "   cor#\n",
        " cor###po#\n",
        "   cor#\n",
        "     cor#\n",
    ), 100.0, "# ")
        + " ...#\n"
        + &fuck_up_text(concat!(
        " co#\n",
        "  co#\n",
        " cor###po#\n",
        "   cor##\n",
        "     cor###po#\n",
        "   cor###po#\n",
        "            cor##\n",
        "      cor###po#\n",
    ), 280.0, "# ")
        + &fuck_up_text(concat!(
        "             cor###po#\n",
        "    cor##\n",
        "                  cor###po#\n",
        "  cor#\n",
        "          cor#\n",
        "                    cor#\n",
    ), 500.0, "# ")
        + &fuck_up_text(concat!(
        " co#\n",
        "                co#\n",
        "     cor###po#\n",
        "                        cor#\n",
        "  cor###po#\n",
        "           cor###po#\n",
        "             cor#\n",
        "  cor###po#\n",
    ), 650.0, "# ")
        + concat!(
        " ???###p?#\n",
        "   ??r#\n",
        "           ???###??#\n",
        "                       co?#\n",
        "                                       ???#\n",
        "....#....#....#....#....#....#....#....#...#...#...#...#..#..\n",
        "              <##C##O##N##N##E##C##T##I##O##N## ##L##O##S##T##>########################################################################################################################################################################################################################################################################################################################################################################################################################################",
    );
    let funding_1: Vec<Vec<String>> = split_word_template(&funding_1);
    broadcasts.set::<Vec<Vec<String>>>("funding_1", funding_1);

    let funding_2: String = "--####--####-- ####-i####-a-####---- ####-up####-o-#\n".to_string()
        + "-n####nu####-l ####fi####nan####cial ####sup####por#\n"
        + &"An####nu####al ####fi####nan####cial ####sup####por#\n".repeat(5)
        + concat!(
        "An####nu###\n",
        "An####nu###\n",
    );
    let funding_2: Vec<Vec<String>> = split_word_template(&funding_2);
    broadcasts.set::<Vec<Vec<String>>>("funding_2", funding_2);

    let fundingx2_0: String = concat!(
        " By\n",
        "  ci\n",
        "    po\n",
        "      po\n",
        "  cor#\n",
        "    cor#\n",
        "        by#\n",
        "          by#\n",
        "rr#rr#rr#ro#oo#oo#aa#aa#aa#\n",
        "##  wers#\n",
        " ble#\n",
        "       b\n",
        "         b\n",
        "           F#\n",
        "         f\n",
        "       fi\n",
        "     i\n",
        "na#aa#aa#aa#aa#aa#aa#aa#\n",
        " Fun\n",
        "Fun####ding#\n",
        " Fu#\n",
        "  Fun#\n",
        " fu#\n",
        "Fun####ding#\n",
        " Fu#\n",
        "  Fun#\n",
        " fu#\n",
        "Fun####ding#\n",
        " Fu#\n",
        "  Fun#\n",
        " fu#\n",
        "Fun####ding#\n",
        "F\n",
        "    Fi\n",
        "   Fin\n",
        "          Fina\n",
        "      Finan\n",
        "po\n",
        "po\n",
        "cor#\n",
        "by#\n",
        "  por\n",
        "Por#tio##\n",
        "Por#tio##\n",
        "Por#tion# nn#nn#nn\n",
        "b\n",
        " b\n",
        "by###\n",
        "---- by###\n",
        "-------- by###\n",
        "------------ by###\n",
        "---------------- by#######\n",
        ">>#>>###\n",
        "By#\n",
        "  by#\n",
        "fi#nan#\n",
        " b#\n",
        "  b#\n",
        "   nc#\n",
        "    nc#\n",
        "Corr#rr#rr#\n",
        "View####ers#### like#### you.#\n",
        "      like#### you.#\n",
        "    like## you.#\n",
        "Fun####ding###\n",
        "Fun####ding###\n",
        "  Fun###\n",
        "    Fu#\n",
        "Fun####ding###\n",
        "  fu#\n",
        "    fu#\n",
        "      fu#\n",
        "        fu#\n",
        "          fu#\n",
        "Cor####por####a####tion\n",
        "The#### cor####por####a####tion#### for#### pub####lic#### broad####cast####ing#### and#### bi####-an####nual#### fii#ii#i\n",
        "of#\n",
        "nan#\n",
        "for#\n",
        "fin####an####cial#### su#\n",
        "for#### fin####an####cial#### ass#ss#ss\n",
        "in#\n",
        "view####ers###\n",
        "you#####\n",
        "| ########This######## is######## P####B####S!############~\n",
    ).to_string();
    let fundingx2_0: Vec<Vec<String>> = split_word_template(&fundingx2_0);
    broadcasts.set::<Vec<Vec<String>>>("fundingx2_0", fundingx2_0);

    let fundingx2_1: String = (0..26)
        .map(|i: i32| format!("{}#### Unknown###", work_out_date(64 * 3390 + i * 64, 0)).replace(".", ".####"))
        .collect::<Vec<String>>().join("\n")
        + "\n| ########This######## is######## P####B####S!############~\n";
    let fundingx2_1: Vec<Vec<String>> = split_word_template(&fundingx2_1);
    broadcasts.set::<Vec<Vec<String>>>("fundingx2_1", fundingx2_1);

    let fundingx2_2: String = "S#e#a#r#c#h#i#n#g# #f#o#r# #a#c#c#e#s##s## ##p##o##i##n##t########.########.########.########\n".to_string()
        + &"Searching for access point########.########.########.########\n".repeat(11)
        + "Found:################ P#B#S# #O#f#f#i#c#i#a#l# #1#1#.#### #R#e#s#t#a#r#t#i#n#g#.####.####.####";
    let fundingx2_2: Vec<Vec<String>> = split_word_template(&fundingx2_2);
    broadcasts.set::<Vec<Vec<String>>>("fundingx2_2", fundingx2_2);

    let fdg_single_0: String = concat!(
        "########.########.########.########.########.########.########.#######\n",
        "########.########.########.########.########.########.########.#######\n",
        "########.########.########.########.########.########.########.#######\n",
        "########.########.########.########.########.########.########.#######\n",
        "########.########.########.########.########.########.########.#######\n",
        "########.########.########.########.########.########.########.#######\n",
        "Fun####ding#### for#### this#### pro####gram#### was#### made#### pos####sible#\n",
        " 7 > > > by###\n",
        " 7 > > > > by###\n",
        " 8 > > > by###\n",
        " 8 > > > > by###\n",
        " 9 > > > by###\n",
        "Fun##\n",
        "     by\n",
        "       by\n",
        "         by\n",
        "           by\n",
        "Funding#\n",
        "     by\n",
        "       by\n",
        "         by\n",
        "           by\n",
        "for\n",
        "thi#i#i#i\n",
        "Pro####gram.#\n",
        "Pro####gram.#\n",
        "Pro\n",
        "  pro\n",
        "    pro\n",
        "      pro\n",
        "Pro####gram.###\n",
        "Fun####ding#### for####\n",
        "            by#\n",
        "              by#\n",
        "Funding for made#### pos####sible#### by#### view####ers#### like#### you.###\n",
        "like#### you.#####\n",
        "##like#### you.#####\n",
        "##like#### you.#####\n",
        "##like#### you.#####\n",
        "##like#### you.#####\n",
        "##like#### you.###\n",
        "Fu\n",
        "  Fu\n",
        "Fun####ding#### for#### this#### pro####gram#### was#### made#### pos####sible#### by#\n",
        "                                         by###\n",
        "                                       by###\n",
        "                                     by###\n",
        "                                   by#\n",
        "Fun###\n",
        "     by\n",
        "       by\n",
        "         by\n",
        "           by\n",
        "Funding#\n",
        "     by\n",
        "       by\n",
        "         by\n",
        "           by\n",
        "for\n",
        "thi#i#i#i\n",
        "Pro####gram.###\n",
        "Pro####gram.###\n",
        "Pro\n",
        "  pro\n",
        "    pro\n",
        "      pro\n",
        "Pro####gram.#\n",
        "Fun####ding#### for####\n",
        "            by#\n",
        "              by#\n",
        "Funding for made#### pos####sible#### by#### view####ers#### like#### you.###\n",
        "like#### you.#####\n",
        "##like#### you.#####\n",
        "##like#### you.#####\n",
        "##like#### you.#####\n",
        "##like#### you.#####\n",
        "####< RET 200################################################################\n",
    ).to_string();
    let fdg_single_0: Vec<Vec<String>> = split_word_template(&fdg_single_0);
    broadcasts.set::<Vec<Vec<String>>>("fdg_single_0", fdg_single_0);

    let fdg_down_0: String = concat!(
        "Fun####ding#### for#### this#### pro####gram#### was#\n",
        "made#### made#### made#### made#### made#### made#### made#### made#### made###\n",
        "pos####sible#### by#### view####ers#### like#### you.#######",
    ).to_string();
    let fdg_down_0: Vec<Vec<String>> = split_word_template(&fdg_down_0);
    broadcasts.set::<Vec<Vec<String>>>("fdg_down_0", fdg_down_0);

    let fdg_down_1: String = concat!(
        "---####----#### ---#### ----#### ---####----#### ---#\n",
        "----#### ----#### ----#### ----#### ----#### ----#### ----#### ----#### ----###\n",
        "---####-----#### --#### ----####---###### ----######## -##-##-##.#######",
    ).to_string();
    let fdg_down_1: Vec<Vec<String>> = split_word_template(&fdg_down_1);
    broadcasts.set::<Vec<Vec<String>>>("fdg_down_1", fdg_down_1);

    let fdg_down_2: String = concat!(
        "Fun####ding#### for#### this#### pro####gram#### was#\n",
        "made#### made#### made#### made#### made#### made#### made#### made#### made###\n",
        "pos####sible#### by#### view####ers###### like######## y##o##u##.#######",
    ).to_string();
    let fdg_down_2: Vec<Vec<String>> = split_word_template(&fdg_down_2);
    broadcasts.set::<Vec<Vec<String>>>("fdg_down_2", fdg_down_2);

    broadcasts
}
