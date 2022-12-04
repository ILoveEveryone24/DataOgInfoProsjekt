use gloo_utils::document;
use yew::prelude::*;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, DataTransfer, Element, HtmlImageElement};
use std::collections::HashMap;
use gloo_net::http::Request;
use futures;
use wasm_bindgen_futures::{JsFuture, future_to_promise, spawn_local};


enum enumClick
{
    //PieceName(xPos, yPos, moveX, moveY, capture?)
    Clicking,
    DraggingStart(DragEvent),
    DraggingOver(DragEvent),
    Dragging(DragEvent),
    ClickedMove(DragEvent),
    ClickedMoveBoard(DragEvent),
    /*
    King,
    Queen,
    Rook,
    Bishop,
    Knight,*/
    //Pawn,
}   

struct PawnOneBL
{
    xMove: i32,
    yMove: i32,
}

struct RookBL
{
    xMove: i32,
    yMove: i32,
}

struct BishopBL
{
    xMove: i32,
    yMove: i32,
}

struct KnightBL
{
    xMove: i32,
    yMove: i32,
}


struct KingBL
{
    xMove: i32,
    yMove: i32,
}

struct PawnOneWH
{
    xMove: i32,
    yMove: i32,
}


struct Clicker
{
    allowClick: bool,
    clicked: bool,
    whiteToMove: bool,
    counter: i64,
    currentDragging: String,
    pieces: HashMap<String, (i32, i32, String, bool)>,
    availableMoves: HashMap<i32, (bool, i32, i32)>,
    availablePieceMovesBlack: Vec<(i32, i32)>,
    availablePieceMovesWhite: Vec<(i32, i32)>,
    pawnOneBL:PawnOneBL,
    kingBL: KingBL,
    rookBL: RookBL,
    bishopBL: BishopBL, 
    knightBL: KnightBL,
    /*pawnTwoBL:PawnTwoBL,
    pawnThreeBL:PawnThreeBL,
    pawnFourBL:PawnFourBL,
    pawnFiveBL:PawnFiveBL,
    pawnSixBL:PawnFiveBL,
    pawnSevenBL:PawnSevenBL,
    pawnEightBL:PawnEightBL,
    
    bishopBL:BishopBL,
    knightBL:KnightBL,
    queenBL:QueenBL,
    kingBL:KingBL,
*/
    pawnOneWH:PawnOneWH,
    /*pawnTwoWH:PawnTwoWH,
    pawnThreeWH:PawnThreeWH,
    pawnFourWH:PawnFourWH,
    pawnFiveWH:PawnFiveWH,
    pawnSixWH:PawnFiveWH,
    pawnSevenWH:PawnSevenWH,
    pawnEightWH:PawnEightWH,
    rookWH:RookWH,
    bishopWH:BishopWH,
    knightWH:KnightWH,
    queenWH:QueenWH,
    kingWH:KingWH,*/
}

impl Component for Clicker
{
    type Message = enumClick;
    type Properties = ();

    fn create(_ctx:&Context<Self>) -> Self
    {
        let mut moves: HashMap<i32, (bool, i32, i32)> = HashMap::with_capacity(64);
        let mut xPos = 0;
        let mut yPos = 0;
        for n in 1..65
        {
            moves.insert(
            n,
            (false,xPos, yPos));
            if(xPos < 700)
            {
                xPos += 100;
            }
            else
            {
                yPos += 100;
                xPos = 0;
            }
        }



        let mut availability: HashMap<i32, (bool, i32, i32)> = HashMap::with_capacity(64);
        let mut xPos = 0;
        let mut yPos = 0;
        Self{
            allowClick : false,
            clicked : false, 
            whiteToMove: true,
            counter : 1000, 
            currentDragging: String::new(),
            pawnOneBL: PawnOneBL{xMove:0, yMove:100},
            pawnOneWH: PawnOneWH{xMove:0, yMove:100},
            rookBL: RookBL{xMove:100, yMove:100},
            kingBL: KingBL{xMove:100, yMove:100},
            bishopBL: BishopBL{xMove:100, yMove:100},
            knightBL:KnightBL{xMove:100, yMove:200},
            pieces: HashMap::from(
            [(String::from("p1"), (0, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p2"), (100, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p3"), (200, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p4"), (300, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p5"), (400, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p6"), (500, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p7"), (600, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p8"), (700, 100, String::from("images/PawnBL.svg"), true)), 

            (String::from("r1"), (0, 0, String::from("images/RookBL.svg"), true)), 
            (String::from("b1"), (200, 0, String::from("images/BishopBL.svg"), true)), 
            (String::from("n1"), (100, 0, String::from("images/KnightBL.svg"), true)), 
            (String::from("q"),  (300, 0, String::from("images/QueenBL.svg"), true)), 
            (String::from("k"),  (400, 0, String::from("images/KingBL.svg"), true)),
            (String::from("r2"), (700, 0, String::from("images/RookBL.svg"), true)), 
            (String::from("b2"), (500, 0, String::from("images/BishopBL.svg"), true)), 
            (String::from("n2"), (600, 0, String::from("images/KnightBL.svg"), true)),

            (String::from("P1"), (0, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P2"), (100, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P3"), (200, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P4"), (300, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P5"), (400, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P6"), (500, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P7"), (600, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P8"), (700, 600, String::from("images/PawnWH.svg"), true)),

            (String::from("R1"), (700, 700, String::from("images/RookWH.svg"), true)), 
            (String::from("B1"), (500, 700, String::from("images/BishopWH.svg"), true)), 
            (String::from("N1"), (600, 700, String::from("images/KnightWH.svg"), true)), 
            (String::from("Q"),  (300, 700, String::from("images/QueenWH.svg"), true)), 
            (String::from("K"),  (400, 700, String::from("images/KingWH.svg"), true)),
            (String::from("R2"), (0, 700, String::from("images/RookWH.svg"), true)), 
            (String::from("B2"), (200, 700, String::from("images/BishopWH.svg"), true)), 
            (String::from("N2"), (100, 700, String::from("images/KnightWH.svg"), true))]),
            //Fix this
            availableMoves: moves,
            availablePieceMovesBlack: Vec::new(),
            availablePieceMovesWhite: Vec::new(),
            /*
            bishopBL:BishopBL{xPos:, yPos:100, xMove, yMove},
            knightBL:KnightBL{xPos:, yPos:100, xMove, yMove},
            queenBL:QueenBL{xPos:, yPos:100, xMove, yMove},
            bishopWH:BishopWH{xPos:, yPos, xMove, yMove},
            knightWH:KnightWH{xPos:, yPos, xMove, yMove},
            queenWH:QueenWH{xPos:, yPos, xMove, yMove},*/
        }
}



    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        self.availablePieceMovesBlack = Vec::new();
        for(key, value) in self.pieces.iter()
        {
            if(key.chars().nth(0).unwrap().is_uppercase() && value.3)
            {
                self.availablePieceMovesBlack.push((value.0, value.1));
            }
        }

        self.availablePieceMovesWhite = Vec::new();
        for(key, value) in self.pieces.iter()
        {
            if(key.chars().nth(0).unwrap().is_lowercase() && value.3)
            {
                self.availablePieceMovesWhite.push((value.0, value.1));
            }
        }

        
        match msg
        {
            enumClick::Clicking => {
                //self.xPos += 100;
                if(self.clicked == false)
                {
                    self.clicked = true;
                    self.counter = 5;
                    true
                }
                else
                {
                    self.clicked = false;
                    self.counter = 1000;
                    true
                }
                
            }

            enumClick::DraggingStart(e) =>
            {
                //Move them and snap them into place
                //Show moves based on their ID
                //e.target().unwrap().unchecked_into::<HtmlInputElement>().set_draggable(true);
                //e.data_transfer().unwrap().set_drop_effect("move");
                //e.data_transfer().unwrap().set_effect_allowed("all");
                //e.target().unwrap().style();
                //let thiss: Element = document().create_element("Jonh").unwrap();
                //e.data_transfer().unwrap().set_drag_image(&thiss, 0, 0);
                //self.xPos += e.client_y();
                //self.yPos += e.client_x() - 500;
                //log!("DragStarted : {:?}", e.data_transfer().unwrap().effect_allowed());
                //log::debug!("Update: {:?}", self.yPos);
                
                //Dont be dumb
                let pieceId = e.target().unwrap().unchecked_into::<HtmlInputElement>().id();
                e.data_transfer().unwrap()
                .set_data("text", e.target().unwrap()
                .unchecked_into::<HtmlInputElement>().id().as_str());

                e.data_transfer().unwrap()
                .set_data("xPos", "10");

                let image = HtmlImageElement::new().unwrap();
                image.set_src(&self.pieces.get(pieceId.as_str()).unwrap().2);
                e.data_transfer().unwrap().set_drag_image(&image,0,0);
                match pieceId.as_str()
                {
                    "p1"|"p2"|"p3"|"p4"|"p5"|"p6"|"p7"|"p8" => 
                    {
                        for(key, value) in self.availableMoves.iter_mut(){
                            value.0 = false;
                        }
                    
                        if(!self.whiteToMove)
                        {
                        let pawnYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1 + self.pawnOneBL.yMove;
                        let pawnxPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let pawnyPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let pawnRightCapture = pawnxPos + 100;
                        let pawnLeftCapture = pawnxPos - 100;
                        let pawnYCapture = pawnyPos + 100; 

                        //log!("{}", );
                        for(key, value) in self.availableMoves.iter_mut(){
                    
                        //pawn
                        //y+100
                            
                            if (value.1 == pawnxPos && value.2 == pawnYMoveForward)
                            && !(self.availablePieceMovesBlack.contains(&(pawnxPos, pawnYMoveForward)))
                            {
                                value.0 = true;
                            }
                            if (value.1 == pawnxPos && value.2 == pawnyPos + pawnYMoveForward && pawnyPos == 100)
                            && !(self.availablePieceMovesBlack.contains(&(pawnxPos, pawnyPos + pawnYMoveForward)))
                            && !(self.availablePieceMovesBlack.contains(&(pawnxPos, pawnYMoveForward)))
                            {
                                value.0 = true;
                            }

                            if ((self.availablePieceMovesBlack.contains(&(pawnRightCapture, pawnYCapture))) && (value.1 == pawnRightCapture) && (value.2 == pawnYCapture)
                            || (self.availablePieceMovesBlack.contains(&(pawnLeftCapture, pawnYCapture))) && (value.1 == pawnLeftCapture) && (value.2 == pawnYCapture))
                            {
                                value.0 = true;
                            }
                        }


                        }


                    },


                    "P1"|"P2"|"P3"|"P4"|"P5"|"P6"|"P7"|"P8" => 
                    {
                        for(key, value) in self.availableMoves.iter_mut(){
                            value.0 = false;
                        }

                        if(self.whiteToMove)
                        {
                        let pawnYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1 - self.pawnOneWH.yMove;
                        let pawnxPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let pawnyPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let pawnRightCapture = pawnxPos + 100;
                        let pawnLeftCapture = pawnxPos - 100;
                        let pawnYCapture = pawnyPos - 100; 
                        for(key, value) in self.availableMoves.iter_mut(){
                    
                        
                            
                            if(value.1 == pawnxPos && value.2 == pawnYMoveForward)
                            && !(self.availablePieceMovesWhite.contains(&(pawnxPos, pawnYMoveForward)))
                            {
                                value.0 = true;
                            }
                            if(value.1 == pawnxPos && value.2 == pawnYMoveForward - self.pawnOneWH.yMove&& pawnyPos == 600)
                            && !(self.availablePieceMovesWhite.contains(&(pawnxPos, pawnYMoveForward - self.pawnOneWH.yMove)))
                            && !(self.availablePieceMovesWhite.contains(&(pawnxPos, pawnYMoveForward)))
                            {
                                value.0 = true;
                            }
                            if ((self.availablePieceMovesWhite.contains(&(pawnRightCapture, pawnYCapture))) && (value.1 == pawnRightCapture) && (value.2 == pawnYCapture)
                            || (self.availablePieceMovesWhite.contains(&(pawnLeftCapture, pawnYCapture))) && (value.1 == pawnLeftCapture) && (value.2 == pawnYCapture))
                            {
                                value.0 = true;
                            }
                        }
                    }

                    },




                    //King start
                    "k"|"K" => 
                    {
                        for(key, value) in self.availableMoves.iter_mut()
                        {
                            value.0 = false;
                        }
                        let kingXMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().0 + self.kingBL.xMove;
                        let kingYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1 + self.kingBL.yMove;
                        let kingXMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().0 - self.kingBL.xMove;
                        let kingYMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().1 - self.kingBL.yMove;
                        let kingxPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let kingyPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;

                        for(key, value) in self.availableMoves.iter_mut(){
                            if(pieceId.as_str() == "k" && !self.whiteToMove){
                                if ((value.1 == kingXMoveForward && value.2 == kingyPos) 
                                && !(self.availablePieceMovesWhite.contains(&(kingXMoveForward, kingyPos))))
    
                                || (value.1 == kingXMoveBackwards && value.2 == kingyPos)
                                && !(self.availablePieceMovesWhite.contains(&(kingXMoveBackwards, kingyPos)))
    
                                || (value.1 == kingxPos && value.2 == kingYMoveForward)
                                && !(self.availablePieceMovesWhite.contains(&(kingxPos, kingYMoveForward)))
    
                                || (value.1 == kingxPos && value.2 == kingYMoveBackwards)
                                && !(self.availablePieceMovesWhite.contains(&(kingxPos, kingYMoveBackwards)))
    
                                || (value.1 == kingXMoveForward && value.2 == kingYMoveForward)
                                && !(self.availablePieceMovesWhite.contains(&(kingXMoveForward, kingYMoveForward)))
    
                                || (value.1 == kingXMoveBackwards && value.2 == kingYMoveBackwards)
                                && !(self.availablePieceMovesWhite.contains(&(kingXMoveBackwards, kingYMoveBackwards))) 
    
                                || (value.1 == kingXMoveForward && value.2 == kingYMoveBackwards)
                                && !(self.availablePieceMovesWhite.contains(&(kingXMoveForward, kingYMoveBackwards)))
    
                                || (value.1 == kingXMoveBackwards && value.2 == kingYMoveForward)
                                && !(self.availablePieceMovesWhite.contains(&(kingXMoveBackwards, kingYMoveForward)))
                                {
                                    value.0 = true;
                                }
                            }
                            else if(pieceId.as_str() == "K" && self.whiteToMove)
                            {
                                if ((value.1 == kingXMoveForward && value.2 == kingyPos) 
                                && !(self.availablePieceMovesBlack.contains(&(kingXMoveForward, kingyPos))))
    
                                || (value.1 == kingXMoveBackwards && value.2 == kingyPos)
                                && !(self.availablePieceMovesBlack.contains(&(kingXMoveBackwards, kingyPos)))
    
                                || (value.1 == kingxPos && value.2 == kingYMoveForward)
                                && !(self.availablePieceMovesBlack.contains(&(kingxPos, kingYMoveForward)))
    
                                || (value.1 == kingxPos && value.2 == kingYMoveBackwards)
                                && !(self.availablePieceMovesBlack.contains(&(kingxPos, kingYMoveBackwards)))
    
                                || (value.1 == kingXMoveForward && value.2 == kingYMoveForward)
                                && !(self.availablePieceMovesBlack.contains(&(kingXMoveForward, kingYMoveForward)))
    
                                || (value.1 == kingXMoveBackwards && value.2 == kingYMoveBackwards)
                                && !(self.availablePieceMovesBlack.contains(&(kingXMoveBackwards, kingYMoveBackwards))) 
    
                                || (value.1 == kingXMoveForward && value.2 == kingYMoveBackwards)
                                && !(self.availablePieceMovesBlack.contains(&(kingXMoveForward, kingYMoveBackwards)))
    
                                || (value.1 == kingXMoveBackwards && value.2 == kingYMoveForward)
                                && !(self.availablePieceMovesBlack.contains(&(kingXMoveBackwards, kingYMoveForward)))
                                {
                                    value.0 = true;
                                }
                            }
                        }
                    },

                    "r1" | "r2" | "R1" | "R2" =>
                    {
                        for(key, value) in self.availableMoves.iter_mut()
                        {
                            value.0 = false;
                        }

                        let mut rookXMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut rookYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let mut rookXMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut rookYMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let rookXPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let rookYPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        if((pieceId.as_str() == "R1" || pieceId.as_str() == "R2") && self.whiteToMove){
                        while(true)
                        {
                            rookYMoveForward += self.rookBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveForward))
                            {
                                rookYMoveForward-= self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveForward))
                            {
                                break;
                            }
                            if rookYMoveForward > 700
                            {
                                rookYMoveForward = 700;
                                break;
                            }
                        }

                        while(true)
                        {
                            rookYMoveBackwards -= self.rookBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                rookYMoveBackwards += self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                break;
                            }
                            if rookYMoveBackwards < 0
                            {
                                rookYMoveBackwards = 0;
                                break;
                            }
                            
                        }
                        
                        while(true)
                        {
                            rookXMoveForward += self.rookBL.xMove;
                            if self.availablePieceMovesBlack.contains(&(rookXMoveForward, rookYPos))
                            {
                                rookXMoveForward-= self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXMoveForward, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveForward > 700
                            {
                                rookXMoveForward = 700;
                                break;
                            }
                        }
                        
                        while(true)
                        {
                            rookXMoveBackwards -= self.rookBL.xMove;
                            if self.availablePieceMovesBlack.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                //THIS X TOOK ME 40 minutes
                                rookXMoveBackwards += self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveBackwards < 0
                            {
                                rookXMoveBackwards = 0;
                                break;
                            }
                        }
                        }
                        //The other
                        else if((pieceId.as_str() == "r1" || pieceId.as_str() == "r2") && !self.whiteToMove)
                        {
                        while(true)
                        {
                            rookYMoveForward += self.rookBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveForward))
                            {
                                rookYMoveForward-= self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveForward))
                            {
                                break;
                            }
                            if rookYMoveForward > 700
                            {
                                rookYMoveForward = 700;
                                break;
                            }
                        }

                        while(true)
                        {
                            rookYMoveBackwards -= self.rookBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                rookYMoveBackwards += self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                break;
                            }
                            if rookYMoveBackwards < 0
                            {
                                rookYMoveBackwards = 0;
                                break;
                            }
                            
                        }
                        
                        while(true)
                        {
                            rookXMoveForward += self.rookBL.xMove;
                            if self.availablePieceMovesWhite.contains(&(rookXMoveForward, rookYPos))
                            {
                                rookXMoveForward-= self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXMoveForward, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveForward > 700
                            {
                                rookXMoveForward = 700;
                                break;
                            }
                        }
                        
                        while(true)
                        {
                            rookXMoveBackwards -= self.rookBL.xMove;
                            if self.availablePieceMovesWhite.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                //THIS X TOOK ME 40 minutes
                                rookXMoveBackwards += self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveBackwards < 0
                            {
                                rookXMoveBackwards = 0;
                                break;
                            }
                        }
                        }
                        for(key, value) in self.availableMoves.iter_mut(){
                            
                            if ((value.1 >= rookXMoveBackwards && value.1 <= rookXMoveForward) && value.1 != rookXPos && value.2 == rookYPos)
                            || (value.1 == rookXPos && (value.2 >= rookYMoveBackwards && value.2 <= rookYMoveForward) && value.2 != rookYPos)
                            {
                                value.0 = true;
                            }
                        }
                    },
                    
                    "b1"|"b2"|"B1"|"B2" =>
                    {
                        for(key, value) in self.availableMoves.iter_mut()
                        {
                            value.0 = false;
                        }

                        let mut bishopXMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let mut bishopXMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().1;

                        let mut bishopXMoveForwardInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveForwardInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let mut bishopXMoveBackwardsInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveBackwardsInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let bishopXPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let bishopYPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        if((pieceId.as_str() == "B1" || pieceId.as_str() == "B2") && self.whiteToMove){
                        while(true)
                        {
                            
                            bishopXMoveForward += self.bishopBL.xMove;
                            bishopYMoveForward += self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                bishopXMoveForward-= self.bishopBL.xMove;
                                bishopYMoveForward-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                break;
                            }
                            if bishopXMoveForward > 700 || bishopYMoveForward > 700
                            {
                                if(bishopXMoveForward > 700)
                                {
                                    bishopXMoveForward = 700;
                                }
                                else
                                {
                                    bishopXMoveForward -= 100;
                                }

                                if(bishopYMoveForward > 700)
                                {
                                    bishopYMoveForward = 700;
                                }
                                else
                                {
                                    bishopYMoveForward -= 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            bishopXMoveBackwards -= self.bishopBL.xMove;
                            bishopYMoveBackwards -= self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                bishopXMoveBackwards+= self.bishopBL.xMove;
                                bishopYMoveBackwards+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                break;
                            }
                            if bishopXMoveBackwards < 0 || bishopYMoveBackwards < 0
                            {
                                if(bishopXMoveBackwards < 0)
                                {
                                    bishopXMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwards += 100;
                                }

                                if(bishopYMoveBackwards < 0)
                                {
                                    bishopYMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopYMoveBackwards += 100;
                                }
                                break;
                            }
                        }
                        
                        

                        while(true)
                        {
                            
                            bishopXMoveForwardInverted += self.bishopBL.xMove;
                            bishopYMoveForwardInverted -= self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                bishopXMoveForwardInverted-= self.bishopBL.xMove;
                                bishopYMoveForwardInverted+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                break;
                            }
                            if bishopXMoveForwardInverted > 700 || bishopYMoveForwardInverted < 0
                            {
                                if(bishopXMoveForwardInverted > 700)
                                {
                                    bishopXMoveForwardInverted = 700;
                                }
                                else
                                {
                                    bishopXMoveForwardInverted -= 100;
                                }

                                if(bishopYMoveForwardInverted < 0)
                                {
                                    bishopYMoveForwardInverted = 0;
                                }
                                else
                                {
                                    bishopYMoveForwardInverted += 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            
                            bishopXMoveBackwardsInverted -= self.bishopBL.xMove;
                            bishopYMoveBackwardsInverted += self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                bishopXMoveBackwardsInverted+= self.bishopBL.xMove;
                                bishopYMoveBackwardsInverted-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                break;
                            }
                            if bishopXMoveBackwardsInverted < 0 || bishopYMoveBackwardsInverted > 700
                            {
                                if(bishopXMoveBackwardsInverted < 0)
                                {
                                    bishopXMoveBackwardsInverted = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwardsInverted += 100;
                                }

                                if(bishopYMoveBackwardsInverted > 700)
                                {
                                    bishopYMoveBackwardsInverted = 700;
                                }
                                else
                                {
                                    bishopYMoveBackwardsInverted -= 100;
                                }
                                break;
                            }
                        }

                    }


                        //hreee
                        else if((pieceId.as_str() == "b1" || pieceId.as_str() == "b2") && !self.whiteToMove)
                        {
                        while(true)
                        {
                            
                            bishopXMoveForward += self.bishopBL.xMove;
                            bishopYMoveForward += self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                bishopXMoveForward-= self.bishopBL.xMove;
                                bishopYMoveForward-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                break;
                            }
                            if bishopXMoveForward > 700 || bishopYMoveForward > 700
                            {
                                if(bishopXMoveForward > 700)
                                {
                                    bishopXMoveForward = 700;
                                }
                                else
                                {
                                    bishopXMoveForward -= 100;
                                }

                                if(bishopYMoveForward > 700)
                                {
                                    bishopYMoveForward = 700;
                                }
                                else
                                {
                                    bishopYMoveForward -= 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            bishopXMoveBackwards -= self.bishopBL.xMove;
                            bishopYMoveBackwards -= self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                bishopXMoveBackwards+= self.bishopBL.xMove;
                                bishopYMoveBackwards+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                break;
                            }
                            if bishopXMoveBackwards < 0 || bishopYMoveBackwards < 0
                            {
                                if(bishopXMoveBackwards < 0)
                                {
                                    bishopXMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwards += 100;
                                }

                                if(bishopYMoveBackwards < 0)
                                {
                                    bishopYMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopYMoveBackwards += 100;
                                }
                                break;
                            }
                        }
                        
                        

                        while(true)
                        {
                            
                            bishopXMoveForwardInverted += self.bishopBL.xMove;
                            bishopYMoveForwardInverted -= self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                bishopXMoveForwardInverted-= self.bishopBL.xMove;
                                bishopYMoveForwardInverted+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                break;
                            }
                            if bishopXMoveForwardInverted > 700 || bishopYMoveForwardInverted < 0
                            {
                                if(bishopXMoveForwardInverted > 700)
                                {
                                    bishopXMoveForwardInverted = 700;
                                }
                                else
                                {
                                    bishopXMoveForwardInverted -= 100;
                                }

                                if(bishopYMoveForwardInverted < 0)
                                {
                                    bishopYMoveForwardInverted = 0;
                                }
                                else
                                {
                                    bishopYMoveForwardInverted += 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            
                            bishopXMoveBackwardsInverted -= self.bishopBL.xMove;
                            bishopYMoveBackwardsInverted += self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                bishopXMoveBackwardsInverted+= self.bishopBL.xMove;
                                bishopYMoveBackwardsInverted-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                break;
                            }
                            if bishopXMoveBackwardsInverted < 0 || bishopYMoveBackwardsInverted > 700
                            {
                                if(bishopXMoveBackwardsInverted < 0)
                                {
                                    bishopXMoveBackwardsInverted = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwardsInverted += 100;
                                }

                                if(bishopYMoveBackwardsInverted > 700)
                                {
                                    bishopYMoveBackwardsInverted = 700;
                                }
                                else
                                {
                                    bishopYMoveBackwardsInverted -= 100;
                                }
                                break;
                            }
                        }
                        }
                        for(key, value) in self.availableMoves.iter_mut(){

                            if ((value.1 >= bishopXMoveBackwards && value.1 <= bishopXMoveForward) && value.1 != bishopXPos 
                            && (bishopXPos - value.1 == bishopYPos - value.2))
                            || ((value.1 >= bishopXMoveBackwardsInverted && value.1 <= bishopXMoveForwardInverted) && value.1 != bishopXPos 
                            && ((bishopXPos - value.1 == (bishopYPos - value.2) * -1) || ((bishopXPos - value.1) * -1 == bishopYPos - value.2)))
                            {
                                value.0 = true;
                            }
                        }
                    },

                    "q"|"Q" =>
                    {
                        for(key, value) in self.availableMoves.iter_mut()
                        {
                            value.0 = false;
                        }

                        let mut rookXMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut rookYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let mut rookXMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut rookYMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let rookXPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let rookYPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        if(pieceId.as_str() == "Q" && self.whiteToMove){
                        while(true)
                        {
                            rookYMoveForward += self.rookBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveForward))
                            {
                                rookYMoveForward-= self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveForward))
                            {
                                break;
                            }
                            if rookYMoveForward > 700
                            {
                                rookYMoveForward = 700;
                                break;
                            }
                        }

                        while(true)
                        {
                            rookYMoveBackwards -= self.rookBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                rookYMoveBackwards += self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                break;
                            }
                            if rookYMoveBackwards < 0
                            {
                                rookYMoveBackwards = 0;
                                break;
                            }
                            
                        }
                        
                        while(true)
                        {
                            rookXMoveForward += self.rookBL.xMove;
                            if self.availablePieceMovesBlack.contains(&(rookXMoveForward, rookYPos))
                            {
                                rookXMoveForward-= self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXMoveForward, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveForward > 700
                            {
                                rookXMoveForward = 700;
                                break;
                            }
                        }
                        
                        while(true)
                        {
                            rookXMoveBackwards -= self.rookBL.xMove;
                            if self.availablePieceMovesBlack.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                //THIS X TOOK ME 40 minutes
                                rookXMoveBackwards += self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveBackwards < 0
                            {
                                rookXMoveBackwards = 0;
                                break;
                            }
                        }
                        }
                        //The other
                        else if(pieceId.as_str() == "q" && !self.whiteToMove)
                        {
                        while(true)
                        {
                            rookYMoveForward += self.rookBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveForward))
                            {
                                rookYMoveForward-= self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveForward))
                            {
                                break;
                            }
                            if rookYMoveForward > 700
                            {
                                rookYMoveForward = 700;
                                break;
                            }
                        }

                        while(true)
                        {
                            rookYMoveBackwards -= self.rookBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                rookYMoveBackwards += self.rookBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXPos, rookYMoveBackwards))
                            {
                                break;
                            }
                            if rookYMoveBackwards < 0
                            {
                                rookYMoveBackwards = 0;
                                break;
                            }
                            
                        }
                        
                        while(true)
                        {
                            rookXMoveForward += self.rookBL.xMove;
                            if self.availablePieceMovesWhite.contains(&(rookXMoveForward, rookYPos))
                            {
                                rookXMoveForward-= self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXMoveForward, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveForward > 700
                            {
                                rookXMoveForward = 700;
                                break;
                            }
                        }
                        
                        while(true)
                        {
                            rookXMoveBackwards -= self.rookBL.xMove;
                            if self.availablePieceMovesWhite.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                //THIS X TOOK ME 40 minutes
                                rookXMoveBackwards += self.rookBL.xMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(rookXMoveBackwards, rookYPos))
                            {
                                break;
                            }
                            if rookXMoveBackwards < 0
                            {
                                rookXMoveBackwards = 0;
                                break;
                            }
                        }
                        }
                        for(key, value) in self.availableMoves.iter_mut(){
                            
                            if ((value.1 >= rookXMoveBackwards && value.1 <= rookXMoveForward) && value.1 != rookXPos && value.2 == rookYPos)
                            || (value.1 == rookXPos && (value.2 >= rookYMoveBackwards && value.2 <= rookYMoveForward) && value.2 != rookYPos)
                            {
                                value.0 = true;
                            }
                        }
                        let mut bishopXMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let mut bishopXMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().1;

                        let mut bishopXMoveForwardInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveForwardInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let mut bishopXMoveBackwardsInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let mut bishopYMoveBackwardsInverted = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        let bishopXPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let bishopYPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;
                        if(pieceId.as_str() == "Q" && self.whiteToMove){
                        while(true)
                        {
                            
                            bishopXMoveForward += self.bishopBL.xMove;
                            bishopYMoveForward += self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                bishopXMoveForward-= self.bishopBL.xMove;
                                bishopYMoveForward-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                break;
                            }
                            if bishopXMoveForward > 700 || bishopYMoveForward > 700
                            {
                                if(bishopXMoveForward > 700)
                                {
                                    bishopXMoveForward = 700;
                                }
                                else
                                {
                                    bishopXMoveForward -= 100;
                                }

                                if(bishopYMoveForward > 700)
                                {
                                    bishopYMoveForward = 700;
                                }
                                else
                                {
                                    bishopYMoveForward -= 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            bishopXMoveBackwards -= self.bishopBL.xMove;
                            bishopYMoveBackwards -= self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                bishopXMoveBackwards+= self.bishopBL.xMove;
                                bishopYMoveBackwards+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                break;
                            }
                            if bishopXMoveBackwards < 0 || bishopYMoveBackwards < 0
                            {
                                if(bishopXMoveBackwards < 0)
                                {
                                    bishopXMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwards += 100;
                                }

                                if(bishopYMoveBackwards < 0)
                                {
                                    bishopYMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopYMoveBackwards += 100;
                                }
                                break;
                            }
                        }
                        
                        

                        while(true)
                        {
                            
                            bishopXMoveForwardInverted += self.bishopBL.xMove;
                            bishopYMoveForwardInverted -= self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                bishopXMoveForwardInverted-= self.bishopBL.xMove;
                                bishopYMoveForwardInverted+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                break;
                            }
                            if bishopXMoveForwardInverted > 700 || bishopYMoveForwardInverted < 0
                            {
                                if(bishopXMoveForwardInverted > 700)
                                {
                                    bishopXMoveForwardInverted = 700;
                                }
                                else
                                {
                                    bishopXMoveForwardInverted -= 100;
                                }

                                if(bishopYMoveForwardInverted < 0)
                                {
                                    bishopYMoveForwardInverted = 0;
                                }
                                else
                                {
                                    bishopYMoveForwardInverted += 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            
                            bishopXMoveBackwardsInverted -= self.bishopBL.xMove;
                            bishopYMoveBackwardsInverted += self.bishopBL.yMove;
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                bishopXMoveBackwardsInverted+= self.bishopBL.xMove;
                                bishopYMoveBackwardsInverted-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                break;
                            }
                            if bishopXMoveBackwardsInverted < 0 || bishopYMoveBackwardsInverted > 700
                            {
                                if(bishopXMoveBackwardsInverted < 0)
                                {
                                    bishopXMoveBackwardsInverted = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwardsInverted += 100;
                                }

                                if(bishopYMoveBackwardsInverted > 700)
                                {
                                    bishopYMoveBackwardsInverted = 700;
                                }
                                else
                                {
                                    bishopYMoveBackwardsInverted -= 100;
                                }
                                break;
                            }
                        }

                    }


                        //hreee
                        else if(pieceId.as_str() == "q" && !self.whiteToMove){
                        while(true)
                        {
                            
                            bishopXMoveForward += self.bishopBL.xMove;
                            bishopYMoveForward += self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                bishopXMoveForward-= self.bishopBL.xMove;
                                bishopYMoveForward-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForward, bishopYMoveForward))
                            {
                                break;
                            }
                            if bishopXMoveForward > 700 || bishopYMoveForward > 700
                            {
                                if(bishopXMoveForward > 700)
                                {
                                    bishopXMoveForward = 700;
                                }
                                else
                                {
                                    bishopXMoveForward -= 100;
                                }

                                if(bishopYMoveForward > 700)
                                {
                                    bishopYMoveForward = 700;
                                }
                                else
                                {
                                    bishopYMoveForward -= 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            bishopXMoveBackwards -= self.bishopBL.xMove;
                            bishopYMoveBackwards -= self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                bishopXMoveBackwards+= self.bishopBL.xMove;
                                bishopYMoveBackwards+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwards, bishopYMoveBackwards))
                            {
                                break;
                            }
                            if bishopXMoveBackwards < 0 || bishopYMoveBackwards < 0
                            {
                                if(bishopXMoveBackwards < 0)
                                {
                                    bishopXMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwards += 100;
                                }

                                if(bishopYMoveBackwards < 0)
                                {
                                    bishopYMoveBackwards = 0;
                                }
                                else
                                {
                                    bishopYMoveBackwards += 100;
                                }
                                break;
                            }
                        }
                        
                        

                        while(true)
                        {
                            
                            bishopXMoveForwardInverted += self.bishopBL.xMove;
                            bishopYMoveForwardInverted -= self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                bishopXMoveForwardInverted-= self.bishopBL.xMove;
                                bishopYMoveForwardInverted+= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveForwardInverted, bishopYMoveForwardInverted))
                            {
                                break;
                            }
                            if bishopXMoveForwardInverted > 700 || bishopYMoveForwardInverted < 0
                            {
                                if(bishopXMoveForwardInverted > 700)
                                {
                                    bishopXMoveForwardInverted = 700;
                                }
                                else
                                {
                                    bishopXMoveForwardInverted -= 100;
                                }

                                if(bishopYMoveForwardInverted < 0)
                                {
                                    bishopYMoveForwardInverted = 0;
                                }
                                else
                                {
                                    bishopYMoveForwardInverted += 100;
                                }
                                break;
                            }
                        }

                        while(true)
                        {
                            
                            bishopXMoveBackwardsInverted -= self.bishopBL.xMove;
                            bishopYMoveBackwardsInverted += self.bishopBL.yMove;
                            if self.availablePieceMovesWhite.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                bishopXMoveBackwardsInverted+= self.bishopBL.xMove;
                                bishopYMoveBackwardsInverted-= self.bishopBL.yMove;
                                break;
                            }
                            if self.availablePieceMovesBlack.contains(&(bishopXMoveBackwardsInverted, bishopYMoveBackwardsInverted))
                            {
                                break;
                            }
                            if bishopXMoveBackwardsInverted < 0 || bishopYMoveBackwardsInverted > 700
                            {
                                if(bishopXMoveBackwardsInverted < 0)
                                {
                                    bishopXMoveBackwardsInverted = 0;
                                }
                                else
                                {
                                    bishopXMoveBackwardsInverted += 100;
                                }

                                if(bishopYMoveBackwardsInverted > 700)
                                {
                                    bishopYMoveBackwardsInverted = 700;
                                }
                                else
                                {
                                    bishopYMoveBackwardsInverted -= 100;
                                }
                                break;
                            }
                        }
                        }
                        for(key, value) in self.availableMoves.iter_mut(){

                            if ((value.1 >= bishopXMoveBackwards && value.1 <= bishopXMoveForward) && value.1 != bishopXPos 
                            && (bishopXPos - value.1 == bishopYPos - value.2))
                            || ((value.1 >= bishopXMoveBackwardsInverted && value.1 <= bishopXMoveForwardInverted) && value.1 != bishopXPos 
                            && ((bishopXPos - value.1 == (bishopYPos - value.2) * -1) || ((bishopXPos - value.1) * -1 == bishopYPos - value.2)))
                            {
                                value.0 = true;
                            }
                        }
                    },

                    "n1"|"n2"|"N1"|"N2"=>
                    {
                        for(key, value) in self.availableMoves.iter_mut()
                        {
                            value.0 = false;
                        }

                        let knightXMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().0 + self.knightBL.xMove;
                        let knightYMoveForward = self.pieces.get_mut(pieceId.as_str()).unwrap().1 + self.knightBL.yMove;
                        let knightXMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().0 - self.knightBL.xMove;
                        let knightYMoveBackwards = self.pieces.get_mut(pieceId.as_str()).unwrap().1 - self.knightBL.yMove;
                        let knightXPos = self.pieces.get_mut(pieceId.as_str()).unwrap().0;
                        let knightYPos = self.pieces.get_mut(pieceId.as_str()).unwrap().1;

                        for(key, value) in self.availableMoves.iter_mut(){
                            if((pieceId.as_str() == "N1" || pieceId.as_str() == "N2") && self.whiteToMove){
                            if (value.1 == knightXPos + self.knightBL.xMove && value.2 == knightYPos - self.knightBL.yMove) 
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos + self.knightBL.xMove, knightYPos - self.knightBL.yMove)))

                            || (value.1 == knightXPos + self.knightBL.yMove && value.2 == knightYPos - self.knightBL.xMove)
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos + self.knightBL.yMove, knightYPos - self.knightBL.xMove)))

                            || (value.1 == knightXPos + self.knightBL.yMove && value.2 == knightYPos + self.knightBL.xMove)
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos + self.knightBL.yMove, knightYPos + self.knightBL.xMove)))

                            || (value.1 == knightXPos + self.knightBL.xMove && value.2 == knightYPos + self.knightBL.yMove)
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos + self.knightBL.xMove, knightYPos + self.knightBL.yMove)))

                            || (value.1 == knightXPos - self.knightBL.xMove && value.2 == knightYPos + self.knightBL.yMove) 
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos - self.knightBL.xMove, knightYPos + self.knightBL.yMove)))

                            || (value.1 == knightXPos - self.knightBL.yMove && value.2 == knightYPos + self.knightBL.xMove)
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos - self.knightBL.yMove, knightYPos + self.knightBL.xMove)))

                            || (value.1 == knightXPos - self.knightBL.yMove && value.2 == knightYPos - self.knightBL.xMove)
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos - self.knightBL.yMove, knightYPos - self.knightBL.xMove)))

                            || (value.1 == knightXPos - self.knightBL.xMove && value.2 == knightYPos - self.knightBL.yMove)
                            && !(self.availablePieceMovesBlack.contains(&(knightXPos - self.knightBL.xMove, knightYPos - self.knightBL.yMove)))
                            {

                                value.0 = true;
                            }
                        }
                        else if ((pieceId.as_str() == "n1" || pieceId.as_str() == "n2") && !self.whiteToMove)
                        {
                            if (value.1 == knightXPos + self.knightBL.xMove && value.2 == knightYPos - self.knightBL.yMove) 
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos + self.knightBL.xMove, knightYPos - self.knightBL.yMove)))

                            || (value.1 == knightXPos + self.knightBL.yMove && value.2 == knightYPos - self.knightBL.xMove)
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos + self.knightBL.yMove, knightYPos - self.knightBL.xMove)))

                            || (value.1 == knightXPos + self.knightBL.yMove && value.2 == knightYPos + self.knightBL.xMove)
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos + self.knightBL.yMove, knightYPos + self.knightBL.xMove)))

                            || (value.1 == knightXPos + self.knightBL.xMove && value.2 == knightYPos + self.knightBL.yMove)
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos + self.knightBL.xMove, knightYPos + self.knightBL.yMove)))

                            || (value.1 == knightXPos - self.knightBL.xMove && value.2 == knightYPos + self.knightBL.yMove) 
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos - self.knightBL.xMove, knightYPos + self.knightBL.yMove)))

                            || (value.1 == knightXPos - self.knightBL.yMove && value.2 == knightYPos + self.knightBL.xMove)
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos - self.knightBL.yMove, knightYPos + self.knightBL.xMove)))

                            || (value.1 == knightXPos - self.knightBL.yMove && value.2 == knightYPos - self.knightBL.xMove)
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos - self.knightBL.yMove, knightYPos - self.knightBL.xMove)))

                            || (value.1 == knightXPos - self.knightBL.xMove && value.2 == knightYPos - self.knightBL.yMove)
                            && !(self.availablePieceMovesWhite.contains(&(knightXPos - self.knightBL.xMove, knightYPos - self.knightBL.yMove)))
                            {

                                value.0 = true;
                            }
                        }
                        }
                    },
                    _=> log!("yes"),
                }
                self.currentDragging = pieceId;
                
                true
            }

            enumClick::DraggingOver(e) =>
            {

                //e.data_transfer().unwrap().set_drop_effect("all");
                //e.data_transfer().unwrap().set_effect_allowed("all");
                e.prevent_default();
                //log!("{}", e.target().unwrap().unchecked_into::<HtmlInputElement>().id());
                //log!("DragOver : {:?}", e.data_transfer());
                //self.pawnOneBL.xPos = e.client_x();
                //self.pawnOneBL.yPos = e.client_y();

                true
            }

            enumClick::Dragging(e) =>
            {
                e.prevent_default();
                //e.data_transfer().unwrap().set_drop_effect("all");
                //e.data_transfer().unwrap().set_effect_allowed("all");
                true
            }

            enumClick::ClickedMove(e) =>
            {
                //FIX THIS
                let mut new_hash = HashMap::new();
                let new_val = wasm_bindgen::JsValue::null();
                for (key, value) in self.pieces.iter()
                {
                    new_hash.insert(key.as_str(), value);
                    let new_val = wasm_bindgen::JsValue::from_f64(value.0 as f64);
                }


                let dataXPos = e.data_transfer().unwrap().get_data("xPos").unwrap();
                
                //futures::executor::block_on(resp);
                //let new_val = wasm_bindgen::JsValue::from_f64();
                //let resp = Request::post("/here").send();
                //let this = JsFuture::from(resp);
                //wasm_bindgen_futures::future_to_promise();
                if(self.whiteToMove)
                {
                    self.whiteToMove = false;
                }
                else
                {
                    self.whiteToMove = true;
                }
                let data = e.data_transfer().unwrap().get_data("text").unwrap();
                let cirId = e.target().unwrap().unchecked_into::<HtmlInputElement>().id().parse::<i32>().unwrap();
                self.pieces.get_mut(&data).unwrap().0 = self.availableMoves.get(&cirId).unwrap().1;
                self.pieces.get_mut(&data).unwrap().1 = self.availableMoves.get(&cirId).unwrap().2;

                self.pieces.get_mut(&data).unwrap().0 = self.availableMoves.get(&cirId).unwrap().1;
                self.pieces.get_mut(&data).unwrap().1 = self.availableMoves.get(&cirId).unwrap().2;
                for(key, value) in self.availableMoves.iter_mut(){
                    value.0 = false;
                }

                //This is where idiotism starts
                use serde::{Serialize, Deserialize};
                #[derive(Serialize, Deserialize)]
                struct testing
                {
                    pieceId : String,
                    pieceDetails : (i32, i32, String, bool),
                }

                let this =
                testing
                {
                    pieceId : data.clone(),
                    pieceDetails : self.pieces.get(&data).unwrap().clone(),
                };

                let resp = Request::post("/here").body(serde_json::to_string(&this).unwrap()).send();
                let this = spawn_local(
                    async{resp.await.unwrap();
                    });

                let piecexPos = self.pieces.get_mut(&data).unwrap().0;;
                let pieceyPos = self.pieces.get_mut(&data).unwrap().1;;
                if(self.availablePieceMovesBlack.contains(&(piecexPos, pieceyPos)))
                    {
                        for(keyTwo, valueTwo) in self.pieces.iter_mut()
                        {
                            if(valueTwo.0 == piecexPos && valueTwo.1 == pieceyPos && keyTwo.chars().nth(0).unwrap().is_uppercase())
                            {
                                valueTwo.3 = false;
                            }
                        }
                }
                if(self.availablePieceMovesWhite.contains(&(piecexPos, pieceyPos)))
                    {
                        for(keyTwo, valueTwo) in self.pieces.iter_mut()
                        {
                            if(valueTwo.0 == piecexPos && valueTwo.1 == pieceyPos && keyTwo.chars().nth(0).unwrap().is_lowercase())
                            {
                                valueTwo.3 = false;
                            }
                        }
                    }

                true
            }

            enumClick::ClickedMoveBoard(e) =>
            {

                e.prevent_default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();
        let startX = -100;
        let startY = -100;
        
        
        html!{

            <div class="container" style="position:relative">
                /*<img src="images/KingBL.svg" onclick={link.callback(|_| enumClick::Clicking)} />
                <p>{self.counter}</p>
                <h1>{"John"}</h1>*/
        //center the thing
        <svg class = "svgBoard" width="800" height="800" viewport="0 0 800 800" 
        //ondrop = {link.callback(|e| enumClick::Dragging(e))} 
        //ondragover={link.callback(|e| enumClick::ClickedMoveBoard(e))}
        
        >
            <rect x="0" y="0" width="100" height = "100" style="fill:#cd9575"/>

            //LOOP THROUGH START POS AND PUT PIECES ON BOARD
            
            <rect x="100" y="0" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="200" y="0"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="300" y="0" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="400" y="0" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="500" y="0"  width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="600" y="0" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="700" y="0"  width="100" height = "100" style ="fill:#3d2b1f"/>


            <rect x="0" y="100" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="100" y="100" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="200" y="100"  width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="300" y="100" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="400" y="100" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="500" y="100"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="600" y="100" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="700" y="100"  width="100" height = "100" style="fill:#cd9575"/>
            
            <rect x="0" y="200" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="100" y="200" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="200" y="200"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="300" y="200" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="400" y="200" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="500" y="200"  width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="600" y="200" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="700" y="200"  width="100" height = "100" style ="fill:#3d2b1f"/>

            <rect x="0" y="300" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="100" y="300" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="200" y="300"  width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="300" y="300" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="400" y="300" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="500" y="300"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="600" y="300" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="700" y="300"  width="100" height = "100" style="fill:#cd9575"/>

            <rect x="0" y="400" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="100" y="400" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="200" y="400"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="300" y="400" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="400" y="400" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="500" y="400"  width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="600" y="400" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="700" y="400"  width="100" height = "100" style ="fill:#3d2b1f"/>

            <rect x="0" y="500" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="100" y="500" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="200" y="500"  width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="300" y="500" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="400" y="500" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="500" y="500"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="600" y="500" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="700" y="500"  width="100" height = "100" style="fill:#cd9575"/>

            <rect x="0" y="600" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="100" y="600" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="200" y="600"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="300" y="600" width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="400" y="600" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="500" y="600"  width="100" height = "100" style ="fill:#3d2b1f"/>
            <rect x="600" y="600" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="700" y="600"  width="100" height = "100" style ="fill:#3d2b1f"/>

            <rect x="0" y="700" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="100" y="700" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="200" y="700"  width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="300" y="700" width="100" height = "100" style="fill:#cd9575"/>
            <rect x="400" y="700" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="500" y="700"  width="100" height = "100" style="fill:#cd9575"/>
            <rect x="600" y="700" width="100" height = "100"  style ="fill:#3d2b1f"/>
            <rect x="700" y="700"  width="100" height = "100" style="fill:#cd9575"/>

            <text x="0" y="25" style={"font-size:25px"} fill = "white">{"8"}</text>
            <text x="0" y="125" style={"font-size:25px"} fill = "white">{"7"}</text>
            <text x="0" y="225" style={"font-size:25px"} fill = "white">{"6"}</text>
            <text x="0" y="325" style={"font-size:25px"} fill = "white">{"5"}</text>
            <text x="0" y="425" style={"font-size:25px"} fill = "white">{"4"}</text>
            <text x="0" y="525" style={"font-size:25px"} fill = "white">{"3"}</text>
            <text x="0" y="625" style={"font-size:25px"} fill = "white">{"2"}</text>
            <text x="0" y="725" style={"font-size:25px"} fill = "white">{"1"}</text>

            <text x="85" y="795" style={"font-size:25px"} fill = "white">{"a"}</text>
            <text x="185" y="795" style={"font-size:25px"} fill = "white">{"b"}</text>
            <text x="285" y="795" style={"font-size:25px"} fill = "white">{"c"}</text>
            <text x="385" y="795" style={"font-size:25px"} fill = "white">{"d"}</text>
            <text x="485" y="795" style={"font-size:25px"} fill = "white">{"e"}</text>
            <text x="585" y="795" style={"font-size:25px"} fill = "white">{"f"}</text>
            <text x="685" y="795" style={"font-size:25px"} fill = "white">{"g"}</text>
            <text x="785" y="795" style={"font-size:25px"} fill = "white">{"h"}</text>

            </svg>

            {self.pieces.iter().map
                (|(key, piece)| html!{<img src={(piece.2).to_string()} 
                    height="100" width="100" 
                    draggable="true"
                    style = {
                        if(piece.3)
                        {
                            format!("top:{}px; left:{}px; position:absolute;visibility:visible;", (piece.1).to_string(), (piece.0 + 317).to_string())
                        }
                        else
                        {
                            format!("top:{}px; left:{}px; position:absolute;visibility:hidden;", (piece.1).to_string(), (piece.0 + 332).to_string())
                        }
                    }

                    x={(piece.0).to_string()} y={(piece.1).to_string()} 
                    id={(key).to_string()} 
                    ondragstart={link.callback(|e| {enumClick::DraggingStart(e)})}
                    />})
                .collect::<Html>()}

            {self.availableMoves.iter().map
                (|(key, value)| html!{
                    <img src="/images/GreenCircle.svg"
                    draggable="false"
                    id = {(key).to_string()}
                    height ="30"    
                    //Move piece there on drop
                    ondrop={link.callback(|e| {enumClick::ClickedMove(e)})}
                    ondragover={link.callback(|e| {enumClick::DraggingOver(e)})}
                    style={
                        if(value.0)
                        {
                            format!("top:{}px;left:{}px;visibility:visible; position:absolute;", value.2 + 35, value.1 + 353)
                        }
                        else
                        {
                            format!("top:{}px;left:{}px;visibility:hidden; position:absolute;", value.2 + 35, value.1 + 353)
                        }
                    }/>
                })
                .collect::<Html>()}
                </div>
        }
    }
}

fn main() {
    //let document = document();
    //let main_container = document.query_selector("#main_container").unwrap().unwrap();

    //yew::start_app_in_element::<App>(main_container);
    yew::start_app::<Clicker>();
}









