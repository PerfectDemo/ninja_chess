
use crate::PIECES;
use crate::STAGE;

struct Command {
    identity: PIECES,
    stage: STAGE,
    from: usize,
    to: usize
}

impl Command {
    fn parse<'a>(command: String) -> Result<Command, &'a str> {
        let mut chars = command.chars();
        let identity;
        let head = chars.next().unwrap();

        match head {
            '车' => identity = PIECES::ROOK,
            '马' => identity = PIECES::ROOK,
            '炮' => identity = PIECES::ROOK,
            '帅' | '将' => identity = PIECES::ROOK,
            '士' => identity = PIECES::ROOK,
            '象' | '相' => identity = PIECES::ROOK,
            '兵' | '卒' => identity = PIECES::PAWN,
            _ => return Err("找不到棋子")
        }

        let from_char = chars.next().unwrap();
        let from;

        match from_char {
            '一' | '1' => from = 1_usize,
            '二' | '2' => from = 2_usize,
            '三' | '3' => from = 3_usize,
            '四' | '4' => from = 4_usize,
            '五' | '5' => from = 5_usize,
            '六' | '6' => from = 6_usize,
            '七' | '7' => from = 7_usize,
            '八' | '8' => from = 8_usize,
            '九' | '9' => from = 9_usize,
            _ => return Err("找不到正确起点")
        } 


        let op = chars.next().unwrap();
        let stage;

        match op {
            '平' => stage = STAGE::TRANSECT,
            '进' => stage = STAGE::FORWARD,
            '退' => stage = STAGE::BACKWARD,
            _ => return Err("请输入正确步骤")
        } 

        let to_char = chars.next().unwrap();
        let to;

        match to_char {
            '一' | '1' => to = 1_usize,
            '二' | '2' => to = 2_usize,
            '三' | '3' => to = 3_usize,
            '四' | '4' => to = 4_usize,
            '五' | '5' => to = 5_usize,
            '六' | '6' => to = 6_usize,
            '七' | '7' => to = 7_usize,
            '八' | '8' => to = 8_usize,
            '九' | '9' => to = 9_usize,
            _ => return Err("找不到正确终点")
        } 

        return Ok(Command {
            identity, from, to, stage
        })

       
    }
}