use esp_hal::{
    gpio::{self, Io, Level, Output, OutputConfig}, spi::{
        self, Mode, master::{Config,Spi}
    }, time::{Duration, Instant,Rate}
};

 pub struct LcdSpi <SPI,RES, DC>{ //template organization here
    spi: SPI,
    res: RES,
    dc: DC,
}

//Rules for the template
impl<SPI,RES, DC> LcdSpi<SPI,RES, DC>
where 
    SPI: spi, //Trait Bound: tells compilier that SPI has to implement the "spi" trait
    res: gpio,
    dc: gpio,
{

}