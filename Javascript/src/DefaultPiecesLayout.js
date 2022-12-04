import React from "react";

import pImg from "./images/PawnBl.svg";
import rImg from "./images/RookBL.svg";
import bImg from "./images/BishopBL.svg";
import nImg from "./images/KnightBL.svg";
import qImg from "./images/QueenBL.svg";
import kImg from "./images/KingBL.svg";

import PImg from "./images/PawnWH.svg";
import RImg from "./images/RookWH.svg";
import BImg from "./images/BishopWH.svg";
import NImg from "./images/KnightWH.svg";
import QImg from "./images/QueenWH.svg";
import KImg from "./images/KingWH.svg";


export const DefaultPieces = {
            p1: {x:0, y:100, xMove:0, yMove:100, src:pImg, visible:true},
            p2:{x:100, y:100, xMove:0, yMove:100, src:pImg, visible:true},
            p3:{x:200, y:100, xMove:0, yMove:100, src:pImg, visible:true},
            p4:{x:300, y:100, xMove:0, yMove:100, src:pImg, visible:true},
            p5:{x:400, y:100, xMove:0, yMove:100, src:pImg, visible:true},
            p6:{x:500, y:100, xMove:0, yMove:100, src:pImg, visible:true},
            p7:{x:600, y:100, xMove:0, yMove:100, src:pImg, visible:true},
            p8:{x:700, y:100, xMove:0, yMove:100, src:pImg, visible:true}, 

            r1: {x:0, y:0, xMove:100, yMove:100, src:rImg, visible: true},
            b1: {x:200, y:0, xMove:100, yMove:100, src:bImg, visible: true}, 
            n1: {x:100, y:0, xMove:100, yMove:200, src:nImg, visible: true},
            q:  {x:300, y:0, xMove:100, yMove:100, src:qImg, visible: true},
            k:  {x:400, y:0, xMove:100, yMove:100, src:kImg, visible: true},
            r2: {x:700, y:0, xMove:100, yMove:100, src:rImg, visible: true},
            b2: {x:500, y:0, xMove:100, yMove:100, src:bImg, visible: true}, 
            n2: {x:600, y:0, xMove:100, yMove:200, src:nImg, visible: true},

            P1: {x:0, y:600, xMove:0, yMove:100, src:PImg,   visible: true},
            P2: {x:100, y:600, xMove:0, yMove:100, src:PImg, visible: true},
            P3: {x:200, y:600, xMove:0, yMove:100, src:PImg, visible: true},
            P4: {x:300, y:600, xMove:0, yMove:100, src:PImg, visible: true},
            P5: {x:400, y:600, xMove:0, yMove:100, src:PImg, visible: true},
            P6: {x:500, y:600, xMove:0, yMove:100, src:PImg, visible: true},
            P7: {x:600, y:600, xMove:0, yMove:100, src:PImg, visible: true},
            P8: {x:700, y:600, xMove:0, yMove:100, src:PImg, visible: true},
            
            R1: {x:700, y:700, xMove:100, yMove:100, src:RImg, visible: true}, 
            B1: {x:500, y:700, xMove:100, yMove:100, src:BImg, visible: true}, 
            N1: {x:600, y:700, xMove:100, yMove:200, src:NImg, visible: true}, 
            Q:  {x:300, y:700, xMove:100, yMove:100, src:QImg, visible: true},
            K:  {x:400, y:700, xMove:100, yMove:100, src:KImg, visible: true},
            R2: {x:0, y:700, xMove:100, yMove:100, src:RImg, visible: true}, 
            B2: {x:200, y:700, xMove:100, yMove:100, src:BImg, visible: true}, 
            N2: {x:100, y:700, xMove:100, yMove:200, src:NImg, visible: true}};