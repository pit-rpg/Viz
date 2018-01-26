extern crate rand;

use std::ops::{Sub, Mul, Add, AddAssign};
// pub use self::ColorTrait;
// use colors;
// use self::rand;
// use std::convert::Into;
// use std::num::Zero;
// trait VecNum: Add + Mul + Sub + Neg + Copy {}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Color <T> {
	pub r: T,
	pub g: T,
	pub b: T,
}


#[allow(dead_code)]
impl <T> Color<T>{}

pub trait ColorTrait <T> {
	// type T;
	// fn zero () -> Self;

	fn copy ( &mut self, color: &Self ) -> &mut Self;
	fn new (r: T, g: T, b: T) -> Self;
	fn random () -> Self;
	fn set_scalar (&mut self, scalar: T ) -> &mut Self;
	fn lerp (  &mut self, color: &Self, alpha: T ) -> &mut Self;
	fn set_rgb ( &mut self, r: T, g: T, b: T ) -> &mut Self;
}


impl <T> ColorTrait<T> for Color<T>
where T: rand::Rand + Sub<Output=T> + AddAssign + Mul<Output=T> + Copy
{

	fn copy ( &mut self, color: &Self ) -> &mut Self {
		self.r = color.r;
		self.g = color.g;
		self.b = color.b;
		self
	}

	fn new (r:T, g:T, b:T) -> Self {
		Self {r, g, b}
	}

	fn random () -> Self {
		 Color {r: rand::random::<T>(), g: rand::random::<T>(), b: rand::random::<T>()}
	}

	fn set_scalar (&mut self, scalar: T ) -> &mut Self {
	    self.r = scalar;
	    self.g = scalar;
	    self.b = scalar;
	    self
	}


	fn lerp (  &mut self, color: &Self, alpha: T ) -> &mut Self {
		self.r += ( color.r - self.r ) * alpha;
		self.g += ( color.g - self.g ) * alpha;
		self.b += ( color.b - self.b ) * alpha;
		self
	}

	fn set_rgb ( &mut self, r: T, g: T, b: T ) -> &mut Self {
		self.r = r;
		self.g = g;
		self.b = b;
		self
	}

	// fn zero () -> Self {
	// 	Color {r: From::from(0) , g: From::from(0), b:  From::from(0)}
	// }
}

impl <T> Add for Color<T>
	where T:  Add<Output = T>, {

	type Output = Color<T>;

	fn add(self, rhs: Color<T>) -> Color<T> {
		Color {
			r: self.r + rhs.r,
			g: self.g + rhs.g,
			b: self.b + rhs.b,
		}
   }
}



// import { _Math } from ./Math.js;
// /**
//  * @author mrdoob / http://mrdoob.com/
//  */
// var ColorKeywords = {  };
// function Color( r, g, b ) {
// 	if ( g === undefined && b === undefined ) {
// 		// r is THREE.Color, hex or string
// 		return this.set( r );
// 	}
// 	return this.setRGB( r, g, b );
// }
// Object.assign( Color.prototype, {
// 	isColor: true,
// 	r: 1, g: 1, b: 1,
// 	set: function ( value ) {
// 		if ( value && value.isColor ) {
// 			this.copy( value );
// 		} else if ( typeof value === 'number' ) {
// 			this.setHex( value );
// 		} else if ( typeof value === 'string' ) {
// 			this.setStyle( value );
// 		}
// 		return this;
// 	},
// 	setHex: function ( hex ) {
// 		hex = Math.floor( hex );
// 		this.r = ( hex >> 16 & 255 ) / 255;
// 		this.g = ( hex >> 8 & 255 ) / 255;
// 		this.b = ( hex & 255 ) / 255;
// 		return this;
// 	},

// 	setHSL: function () {
// 		function hue2rgb( p, q, t ) {
// 			if ( t < 0 ) t += 1;
// 			if ( t > 1 ) t -= 1;
// 			if ( t < 1 / 6 ) return p + ( q - p ) * 6 * t;
// 			if ( t < 1 / 2 ) return q;
// 			if ( t < 2 / 3 ) return p + ( q - p ) * 6 * ( 2 / 3 - t );
// 			return p;
// 		}
// 		return function setHSL( h, s, l ) {
// 			// h,s,l ranges are in 0.0 - 1.0
// 			h = _Math.euclideanModulo( h, 1 );
// 			s = _Math.clamp( s, 0, 1 );
// 			l = _Math.clamp( l, 0, 1 );
// 			if ( s === 0 ) {
// 				this.r = this.g = this.b = l;
// 			} else {
// 				var p = l <= 0.5 ? l * ( 1 + s ) : l + s - ( l * s );
// 				var q = ( 2 * l ) - p;
// 				this.r = hue2rgb( q, p, h + 1 / 3 );
// 				this.g = hue2rgb( q, p, h );
// 				this.b = hue2rgb( q, p, h - 1 / 3 );
// 			}
// 			return this;
// 		};
// 	}(),
// 	setStyle: function ( style ) {
// 		function handleAlpha( string ) {
// 			if ( string === undefined ) return;
// 			if ( parseFloat( string ) < 1 ) {
// 				console.warn( 'THREE.Color: Alpha component of ' + style + ' will be ignored.' );
// 			}
// 		}
// 		var m;
// 		if ( m = /^((?:rgb|hsl)a?)\(\s*([^\)]*)\)/.exec( style ) ) {
// 			// rgb / hsl
// 			var color;
// 			var name = m[ 1 ];
// 			var components = m[ 2 ];
// 			switch ( name ) {
// 				case 'rgb':
// 				case 'rgba':
// 					if ( color = /^(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(,\s*([0-9]*\.?[0-9]+)\s*)?$/.exec( components ) ) {
// 						// rgb(255,0,0) rgba(255,0,0,0.5)
// 						this.r = Math.min( 255, parseInt( color[ 1 ], 10 ) ) / 255;
// 						this.g = Math.min( 255, parseInt( color[ 2 ], 10 ) ) / 255;
// 						this.b = Math.min( 255, parseInt( color[ 3 ], 10 ) ) / 255;
// 						handleAlpha( color[ 5 ] );
// 						return this;
// 					}
// 					if ( color = /^(\d+)\%\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(,\s*([0-9]*\.?[0-9]+)\s*)?$/.exec( components ) ) {
// 						// rgb(100%,0%,0%) rgba(100%,0%,0%,0.5)
// 						this.r = Math.min( 100, parseInt( color[ 1 ], 10 ) ) / 100;
// 						this.g = Math.min( 100, parseInt( color[ 2 ], 10 ) ) / 100;
// 						this.b = Math.min( 100, parseInt( color[ 3 ], 10 ) ) / 100;
// 						handleAlpha( color[ 5 ] );
// 						return this;
// 					}
// 					break;
// 				case 'hsl':
// 				case 'hsla':
// 					if ( color = /^([0-9]*\.?[0-9]+)\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(,\s*([0-9]*\.?[0-9]+)\s*)?$/.exec( components ) ) {
// 						// hsl(120,50%,50%) hsla(120,50%,50%,0.5)
// 						var h = parseFloat( color[ 1 ] ) / 360;
// 						var s = parseInt( color[ 2 ], 10 ) / 100;
// 						var l = parseInt( color[ 3 ], 10 ) / 100;
// 						handleAlpha( color[ 5 ] );
// 						return this.setHSL( h, s, l );
// 					}
// 					break;
// 			}
// 		} else if ( m = /^\#([A-Fa-f0-9]+)$/.exec( style ) ) {
// 			// hex color
// 			var hex = m[ 1 ];
// 			var size = hex.length;
// 			if ( size === 3 ) {
// 				// #ff0
// 				this.r = parseInt( hex.charAt( 0 ) + hex.charAt( 0 ), 16 ) / 255;
// 				this.g = parseInt( hex.charAt( 1 ) + hex.charAt( 1 ), 16 ) / 255;
// 				this.b = parseInt( hex.charAt( 2 ) + hex.charAt( 2 ), 16 ) / 255;
// 				return this;
// 			} else if ( size === 6 ) {
// 				// #ff0000
// 				this.r = parseInt( hex.charAt( 0 ) + hex.charAt( 1 ), 16 ) / 255;
// 				this.g = parseInt( hex.charAt( 2 ) + hex.charAt( 3 ), 16 ) / 255;
// 				this.b = parseInt( hex.charAt( 4 ) + hex.charAt( 5 ), 16 ) / 255;
// 				return this;
// 			}
// 		}
// 		if ( style && style.length > 0 ) {
// 			// color keywords
// 			var hex = ColorKeywords[ style ];
// 			if ( hex !== undefined ) {
// 				// red
// 				this.setHex( hex );
// 			} else {
// 				// unknown color
// 				console.warn( 'THREE.Color: Unknown color ' + style );
// 			}
// 		}
// 		return this;
// 	},
// 	clone: function () {
// 		return new this.constructor( this.r, this.g, this.b );
// 	},
// 	copy: function ( color ) {
// 		this.r = color.r;
// 		this.g = color.g;
// 		this.b = color.b;
// 		return this;
// 	},
// 	copyGammaToLinear: function ( color, gammaFactor ) {
// 		if ( gammaFactor === undefined ) gammaFactor = 2.0;
// 		this.r = Math.pow( color.r, gammaFactor );
// 		this.g = Math.pow( color.g, gammaFactor );
// 		this.b = Math.pow( color.b, gammaFactor );
// 		return this;
// 	},
// 	copyLinearToGamma: function ( color, gammaFactor ) {
// 		if ( gammaFactor === undefined ) gammaFactor = 2.0;
// 		var safeInverse = ( gammaFactor > 0 ) ? ( 1.0 / gammaFactor ) : 1.0;
// 		this.r = Math.pow( color.r, safeInverse );
// 		this.g = Math.pow( color.g, safeInverse );
// 		this.b = Math.pow( color.b, safeInverse );
// 		return this;
// 	},
// 	convertGammaToLinear: function () {
// 		var r = this.r, g = this.g, b = this.b;
// 		this.r = r * r;
// 		this.g = g * g;
// 		this.b = b * b;
// 		return this;
// 	},
// 	convertLinearToGamma: function () {
// 		this.r = Math.sqrt( this.r );
// 		this.g = Math.sqrt( this.g );
// 		this.b = Math.sqrt( this.b );
// 		return this;
// 	},
// 	getHex: function () {
// 		return ( this.r * 255 ) << 16 ^ ( this.g * 255 ) << 8 ^ ( this.b * 255 ) << 0;
// 	},
// 	getHexString: function () {
// 		return ( '000000' + this.getHex().toString( 16 ) ).slice( - 6 );
// 	},
// 	getHSL: function ( optionalTarget ) {
// 		// h,s,l ranges are in 0.0 - 1.0
// 		var hsl = optionalTarget || { h: 0, s: 0, l: 0 };
// 		var r = this.r, g = this.g, b = this.b;
// 		var max = Math.max( r, g, b );
// 		var min = Math.min( r, g, b );
// 		var hue, saturation;
// 		var lightness = ( min + max ) / 2.0;
// 		if ( min === max ) {
// 			hue = 0;
// 			saturation = 0;
// 		} else {
// 			var delta = max - min;
// 			saturation = lightness <= 0.5 ? delta / ( max + min ) : delta / ( 2 - max - min );
// 			switch ( max ) {
// 				case r: hue = ( g - b ) / delta + ( g < b ? 6 : 0 ); break;
// 				case g: hue = ( b - r ) / delta + 2; break;
// 				case b: hue = ( r - g ) / delta + 4; break;
// 			}
// 			hue /= 6;
// 		}
// 		hsl.h = hue;
// 		hsl.s = saturation;
// 		hsl.l = lightness;
// 		return hsl;
// 	},
// 	getStyle: function () {
// 		return 'rgb(' + ( ( this.r * 255 ) | 0 ) + ',' + ( ( this.g * 255 ) | 0 ) + ',' + ( ( this.b * 255 ) | 0 ) + ')';
// 	},
// 	offsetHSL: function ( h, s, l ) {
// 		var hsl = this.getHSL();
// 		hsl.h += h; hsl.s += s; hsl.l += l;
// 		this.setHSL( hsl.h, hsl.s, hsl.l );
// 		return this;
// 	},
// 	add: function ( color ) {
// 		this.r += color.r;
// 		this.g += color.g;
// 		this.b += color.b;
// 		return this;
// 	},
// 	addColors: function ( color1, color2 ) {
// 		this.r = color1.r + color2.r;
// 		this.g = color1.g + color2.g;
// 		this.b = color1.b + color2.b;
// 		return this;
// 	},
// 	addScalar: function ( s ) {
// 		this.r += s;
// 		this.g += s;
// 		this.b += s;
// 		return this;
// 	},
// 	sub: function ( color ) {
// 		this.r = Math.max( 0, this.r - color.r );
// 		this.g = Math.max( 0, this.g - color.g );
// 		this.b = Math.max( 0, this.b - color.b );
// 		return this;
// 	},
// 	multiply: function ( color ) {
// 		this.r *= color.r;
// 		this.g *= color.g;
// 		this.b *= color.b;
// 		return this;
// 	},
// 	multiplyScalar: function ( s ) {
// 		this.r *= s;
// 		this.g *= s;
// 		this.b *= s;
// 		return this;
// 	},

// 	equals: function ( c ) {
// 		return ( c.r === this.r ) && ( c.g === this.g ) && ( c.b === this.b );
// 	},
// 	fromArray: function ( array, offset ) {
// 		if ( offset === undefined ) offset = 0;
// 		this.r = array[ offset ];
// 		this.g = array[ offset + 1 ];
// 		this.b = array[ offset + 2 ];
// 		return this;
// 	},
// 	toArray: function ( array, offset ) {
// 		if ( array === undefined ) array = [];
// 		if ( offset === undefined ) offset = 0;
// 		array[ offset ] = this.r;
// 		array[ offset + 1 ] = this.g;
// 		array[ offset + 2 ] = this.b;
// 		return array;
// 	},
// 	toJSON: function () {
// 		return this.getHex();
// 	}
// } );
// export { Color };
