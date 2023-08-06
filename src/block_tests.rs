#[cfg(test)]
mod block_tests {
    use std::{fmt::Write, time::Instant};

    use crate::block::Block;

    //check the different hash functions of part 2 Block
    #[test]
    fn hash(){
        const VALID_HASH_STRING : &str = "6c71ff02a08a22309b7dbbcee45d291d4ce955caa32031c50d941e3e9dbd0000:1:16:message:2159";
        const VALID_HASH: &str = "9b4417b36afa6d31c728eed7abc14dd84468fdb055d8f3cbe308b0179df40000";
        
        // test is_valid()
        let mut b3 = Block::initial(16);
        assert_eq!(b3.is_valid_for_proof(0), false);
        assert_eq!(b3.is_valid_for_proof(56231),true);

        b3.set_proof(56231);
        assert_eq!(b3.is_valid(),true);

        let mut b1 = Block::next(&b3, String::from("message"));
        b1.set_proof(2159);


        //test hash_string and hash
        assert_eq!(b1.hash_string(), VALID_HASH_STRING);

        let mut output = String::new();
        write!(&mut output, "{:02x}", b1.hash()).unwrap();
        assert_eq!(output, VALID_HASH);
    }
    // the remaining tests are for part 3 Mining
    #[test]
    fn mine_difficulity_7(){
        const VALID_HASH_STRING: & str = "4a1c722d8021346fa2f440d7f0bbaa585e632f68fd20fed812fc944613b92500:2:7:this is not interesting:40";
        const VALID_HASH : &str = "ba2f9bf0f9ec629db726f1a5fe7312eb76270459e3f5bfdc4e213df9e47cd380";

        let mut b0 = Block::initial(7);
        b0.mine(100);
        let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
        b1.mine(100);
        let mut b2 = Block::next(&b1, String::from("this is not interesting"));
        b2.mine(100);

        assert_eq!(b2.is_valid(), true);
    }

    #[test]
    fn mine_difficulity_20(){
        const VALID_HASH_STRING: & str = "a42b7e319ee2dee845f1eb842c31dac60a94c04432319638ec1b9f989d000000:2:20:this is not interesting:1017262";
        const VALID_HASH : &str = "6c589f7a3d2df217fdb39cd969006bc8651a0a3251ffb50470cbc9a0e4d00000";

        let mut b0 = Block ::initial(20);
        b0.mine(100);
        let mut b1: Block = Block::next(&b0, String::from("this is an interesting message"));
        b1.mine(100);
        let mut b2 = Block::next(&b1, String::from("this is not interesting"));
        b2.mine(100);

        assert_eq!(b2.is_valid(), true);
    }

    // test 100 workers vs 1 workers for difficulty 20
    #[test]
    fn miners_speed(){
        let start_1 = Instant::now();
        let mut b0 = Block::initial(20);
        b0.mine(1);   
        let end_1 = Instant::now();

        let start_100 = Instant::now();
        let mut b1 = Block::initial(20);
        b1.mine(100);          
        let end_100 = Instant::now();

        let time_1 = end_1.duration_since(start_1).as_millis();
        let time_100 = end_100.duration_since(start_100).as_millis();
        

        assert!(time_1 > time_100);
    }

}