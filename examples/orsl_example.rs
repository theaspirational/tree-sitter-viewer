extern crate tree_sitter_orsl; 
extern crate tree_sitter_viewer;

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {

    // initial code to display
    let code = r#"
namespace module1:

	context global:
		user-variables:
			Map eg = loadMapFile("effectiveGroups.csv")


	context client_order:
		user-variables:
			Bool discounted = False
		user-marks:
			String wheel
		
		@Entry
		procedure ARCA(param1="defaultvalue"):
			runif:
				co.Dest == "ARCA"
				co.TIF in [0,3]
				or:
					co.order_type != "Market"
					co.session == "MAIN"
			init:
				setDefaults()
				co.isALO = isAllAlo(final_sequence)
				co.replace_start_over = "Y"
			sequence:
				send("GSARCAIOC")
				ARCA_extra()
				send("DIARCAIOC")
				if (param1=="test"):
					send("GSARCA")
				send("JPMARCA") # comment
		
		procedure ARCA_extra():
			runif:
				sym.session == "MAIN"
			sequence:
				send("SPARCA")
		
		@Entry
		procedure ARCA2(param1):
			runif:
				co.Dest == "ARCA2"
			sequence:
				ARCA(param1):
					loc = removeSend("GSARCAIOC")
					insertAt(loc):
						send("FXARCA")
						send("GSARCA")

		@Entry
		procedure ARCA2():
			runif:
				eg[co.TraderGroup] == "COPIA_METAGROUP_1"
			sequence:
				co.discounted = True
				ARCA3("projection_val")

		@Entry
		procedure ARCA2():
			runif:
				co.sym.ADV > 50000
			sequence:
				ARCA3("another value")

		@Entry
		procedure SplitTest():
			runif:
				co.Dest == "SPLIT"
			sequence:
				SeqList seq_arr = [ARCA2(), ARCA3(), ARCA4(), ARCA5(), ARCA6()]
				randomize(seq_arr)
				fills = split([0.25, 0.25, 0.25, 0.25], seq_arr)
				sort_by_size(fills)
				fills[0].seq()
				ARCA_extra()

		@Entry
		procedure ARCA5():
			runif:
				co.Dest == "ARCA5"
			sequence:
				ARCA2()
				ARCA3()
				delay(co.remaining < co.size / 2, 3.5)
				ARCA4()

		@Entry
		procedure AnyTest():
			runif:
				co.Dest == "ANY"
			sequence:
				ARCA2()
				any(must_accept=True): 
					ARCA3()
					ARCA4()
				ARCA5()

		@Entry
		procedure MarksTest1():
			runif:
				co.Dest=="MARKS"
			sequence:
				ARCA2()
				mark(wheel="WHEEL1")
				ARCA3()
				mark(wheel="SUBWHEEL_1A")
				ARCA4()
				ARCA5()
				unmark(wheel="SUBWHEEL_1A")
				ARCA6()
				unmark(wheel="WHEEL1")

		@Entry
		procedure MarksTest2():
			runif:
				co.Dest=="MARKS2"
			sequence:
				ARCA2()
				mark(wheel="WHEEL1")
				ARCA3()
				mark(wheel="WHEEL2")
				ARCA4()
				unmark(wheel="WHEEL1")
				ARCA5()
				ARCA6()
				unmark(wheel="WHEEL2")
				ARCA7()

		procedure SRRP():
			sequence:
				sendAsRPIOC(vo.JPMIEX, midPeg=True)

		procedure SROUTERARCA():
			sequence:
				mark(wheel="SROUTERARCA")
				sendIOC(HRTDARKREB)
				sendIOC(CITADARKRE)
				sendIOC(PDQDARKREB)
				sendIOC(JPMBYXTRIM)
				sendIOC(JPMDARK)
				unmark(wheel="SROUTERARCA")

		@Entry
		procedure realARCA():
			init:
				co.replace_start_over = "Y"
				or:
					mkt.session == "MAIN"
					co.type != ext.MARKET
			sequence:
				SRRP()
				SROUTERARCA()
				sendIOC(GSARCA)
				sendIOC(DIARCA)
				arcaFinal()

		procedure arcaFinal():
			sequence:
				any:
					send(FXARCA, type=vo.type, hidden=vo.hidden)
					send(GSARCA, type=vo.type, hidden=vo.hidden)
					send(JPMARCA, type=vo.type, hidden=vo.hidden)
					send(INARCA, type=vo.type, hidden=vo.hidden)
					send(LMARCA, type=vo.type, hidden=vo.hidden)
					send(SPARCA, type=vo.type, hidden=vo.hidden)
					send(PDARCAMV, type=vo.type, hidden=vo.hidden)
					send(DIARCA, type=vo.type, hidden=vo.hidden)


	context venue_order:

		procedure color_palette_base():
			map:
				vo.venue: "COLOR_PALETTE_F"
				vo.type: "LIMIT"
				vo.TIF: "TIF_DAY"

		@Entry
		procedure FXARCA(hidden=False, ioc=False, type=ext.LIMIT):
			init:
				if sym.listed != "U":
					abort()
			map:
				color_pallete_base()
				vo.exchange: "ARCA"
				crossable: "Y"
				vo.type: type
				
				if hidden == False:
					vo.EX_DESTINATION: "RRDA"
					vo.routable: "Y"
				if hidden == True:
					vo.EX_DESTINATION: "RRZZ"
					vo.routable: "N"
					vo.dark: "Y"
				vo.TIF = ioc?"TIF_IOC":"TIF_DAY"

		@Entry
		procedure FXARCAMKT(hidden=False):
			map:
				FXARCA(hidden)
				type: "MARKET"

		@Entry
		procedure FXARCAPRE(hidden=False):
			runif:
				env.time > "4:00"
				env.time < "9:30"
			map:
				FXARCA(ioc=False, ...)
		
		procedure gs_dma_base():
			init:
				if sym.ADV < 1000000:
					abort()
			map:
				venue: ext.GS_DMA_F
				type: ext.LIMIT
				TIF: ext.TIF_DAY

		@Entry
		procedure GSARCA(tif=ext.TIF_DAY, hidden=False, maxFloorPass=False, alo=False):
			init:
				vo.routable = !hidden & !maxFloorPass
				vo.dark = vo.routable
				vo.alo = alo
			map:
				gs_dma_base()
				EX_DESTINATION: "ARCX"
				TRADING_SESSIONS: ext.TRADING_SESSIONS_PREOPEN_AND_DAY_AND_EXTENDED_HOURS
				crossable: ext.Y
				destination_venue: "ARCA"
				routable: ext.Y
				exchange: mkt.ARCA
				exclude_listed: sym.U
				TIF: tif
				if hidden & alo:
					EX_INSTR: "pl"
				elif hidden:
					EX_INSTR: "xx"
				elif alo:
					EX_INSTR: "zz"
				else:
					EX_INSTR: None
				MAX_FLOOR: maxFloorPass?vo.max_floor:None


		@Entry
		procedure JPMARCA(type=ext.LIMIT, hidden=False):
			sequence:
				if type==ext.MARKET:
					vo.type: ext.MARKET
					vo.tag[386]: 1
					vo.tag[336]: 2
				else:
					vo.type=ext.LIMIT


		@Entry
		procedure JPMIEX():
			runif:
				vo.oddlot == False
				or:
					sym.adv > 50000
					co.MPID in goodGroup
			# map:
				# stuff goes here

	context venue_fill:

		function String copiaLiqFlag(exchange, venue_liq, discounted=False, passive=False):
			result = supermap(venue_liq):
				exchange in ["ARCA","GSARCA"]:
					"AB" => "a"
					"Ab" => "a"
					discounted:
						"A" => "b"
					!discounted:
						"A" => "a"
					"B*" => "b"
				excahnge=="NASDAQ";passive:
					"CD" => "c"
					"Cd" => "d"
					=> "a"
				exchange=="NASDAQ";(discounted|passive):
					"AB" => "a"
					"Ab" => "a"
				=> "z"
			return result
		

		@Entry
		procedure CopiaMain():
			runif:
				eg[co.TraderGroup] in "COPIA_METAGROUP_1"
			map:
				co_fill.liq_flag = copiaLiqFlag(vo.exchange, vfill.venue_liq, co.discounted, vo.passive)
				co_fill.tag[9912] = '123'
"#;

    // specify the parser's language and the initial code.
    let result = tree_sitter_viewer::run(tree_sitter_orsl::language(), code ); 

    result.await
}
