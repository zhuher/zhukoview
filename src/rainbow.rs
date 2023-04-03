#![no_std]
enum Colourd {
    Redplusgreen,
    Greenminusred,
    Greenplusblue,
    Blueminusgreen,
    Blueplusred,
    Redminusblue,
}
pub struct Rainbow {
    pub colour: u32,
    dn: u32,
    cd: Colourd,
}
impl Rainbow {
    pub fn new() -> Self {
        Self {
            colour: 0xFF0000FF,
            dn: 0x10000,
            cd: Colourd::Redplusgreen,
        }
    }
}
impl Default for Rainbow {
    #[inline(always)]
    fn default() -> Self {
        Rainbow::new()
    }
}
impl Iterator for Rainbow {
    type Item = u32;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.cd {
            Colourd::Redplusgreen => {
                self.colour = self.colour.saturating_add(0x010000);
                if self.colour >> 16 & 0xFF == 0xFF {
                    self.cd = Colourd::Greenminusred;
                }
                Some(self.colour)
            }
            Colourd::Greenminusred => {
                self.colour = self.colour.saturating_sub(0x01000000);
                if self.colour >> 24 & 0xFF == 0x0 {
                    self.cd = Colourd::Greenplusblue;
                }
                Some(self.colour)
            }
            Colourd::Greenplusblue => {
                self.colour = self.colour.saturating_add(0x0100);
                if self.colour >> 8 & 0xFF == 0xFF {
                    self.cd = Colourd::Blueminusgreen;
                }
                Some(self.colour)
            }
            Colourd::Blueminusgreen => {
                self.colour = self.colour.saturating_sub(0x010000);
                if self.colour >> 16 & 0xFF == 0x0 {
                    self.cd = Colourd::Blueplusred;
                }
                Some(self.colour)
            }
            Colourd::Blueplusred => {
                self.colour = self.colour.saturating_add(0x01000000);
                if self.colour >> 24 & 0xFF == 0xFF {
                    self.cd = Colourd::Redminusblue;
                }
                Some(self.colour)
            }
            Colourd::Redminusblue => {
                self.colour = self.colour.saturating_sub(0x0100);
                if self.colour >> 8 & 0xFF == 0x0 {
                    self.cd = Colourd::Redplusgreen;
                }
                Some(self.colour)
            }
        }
    }
}
