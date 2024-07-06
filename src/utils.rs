use std::fmt;

use bevy_rapier3d::rapier::prelude::ShapeType;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
#[derive(Debug, Clone)]
pub struct ShapeTypeSerializable(pub ShapeType);
impl Serialize for ShapeTypeSerializable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        match self.0 {
            ShapeType::Ball => serializer.serialize_str("Ball"),
            ShapeType::Cuboid => serializer.serialize_str("Cuboid"),
            ShapeType::Capsule => serializer.serialize_str("Capsule"),
            ShapeType::Segment => serializer.serialize_str("Segment"),
            ShapeType::Triangle => serializer.serialize_str("Triangle"),
            ShapeType::TriMesh => serializer.serialize_str("TriMesh"),
            ShapeType::Polyline => serializer.serialize_str("Polyline"),
            ShapeType::HalfSpace => serializer.serialize_str("HalfSpace"),
            ShapeType::HeightField => serializer.serialize_str("HeightField"),
            ShapeType::Compound => serializer.serialize_str("Compound"),
            ShapeType::ConvexPolyhedron => serializer.serialize_str("ConvexPolyhedron"),
            ShapeType::Cylinder => serializer.serialize_str("Cylinder"),
            ShapeType::Cone => serializer.serialize_str("Cone"),
            ShapeType::RoundCuboid => serializer.serialize_str("RoundCuboid"),
            ShapeType::RoundTriangle => serializer.serialize_str("RoundTriangle"),
            ShapeType::RoundCylinder => serializer.serialize_str("RoundCylinder"),
            ShapeType::RoundCone => serializer.serialize_str("RoundCone"),
            ShapeType::RoundConvexPolyhedron => serializer.serialize_str("RoundConvexPolyhedron"),
            ShapeType::Custom => serializer.serialize_str("Custom"),
        }
    }
}
impl<'de> Deserialize<'de> for ShapeTypeSerializable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ShapeTypeSerializableVisitor;

        impl<'de> Visitor<'de> for ShapeTypeSerializableVisitor {
            type Value = ShapeTypeSerializable;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an enum variant")
            }

            fn visit_str<E>(self, value: &str) -> Result<ShapeTypeSerializable, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Ball" => Ok(ShapeTypeSerializable(ShapeType::Ball)),
                    "Cuboid" => Ok(ShapeTypeSerializable(ShapeType::Cuboid)),
                    "Capsule" => Ok(ShapeTypeSerializable(ShapeType::Capsule)),
                    "Segment" => Ok(ShapeTypeSerializable(ShapeType::Segment)),
                    "Triangle" => Ok(ShapeTypeSerializable(ShapeType::Triangle)),
                    "TriMesh" => Ok(ShapeTypeSerializable(ShapeType::TriMesh)),
                    "Polyline" => Ok(ShapeTypeSerializable(ShapeType::Polyline)),
                    "HalfSpace" => Ok(ShapeTypeSerializable(ShapeType::HalfSpace)),
                    "HeightField" => Ok(ShapeTypeSerializable(ShapeType::HeightField)),
                    "Compound" => Ok(ShapeTypeSerializable(ShapeType::Compound)),
                    "ConvexPolyhedron" => Ok(ShapeTypeSerializable(ShapeType::ConvexPolyhedron)),
                    "Cylinder" => Ok(ShapeTypeSerializable(ShapeType::Cylinder)),
                    "Cone" => Ok(ShapeTypeSerializable(ShapeType::Cone)),
                    "RoundCuboid" => Ok(ShapeTypeSerializable(ShapeType::RoundCuboid)),
                    "RoundTriangle" => Ok(ShapeTypeSerializable(ShapeType::RoundTriangle)),
                    "RoundCylinder" => Ok(ShapeTypeSerializable(ShapeType::RoundCylinder)),
                    "RoundCone" => Ok(ShapeTypeSerializable(ShapeType::RoundCone)),
                    "RoundConvexPolyhedron" => {
                        Ok(ShapeTypeSerializable(ShapeType::RoundConvexPolyhedron))
                    }
                    "Custom" => Ok(ShapeTypeSerializable(ShapeType::Custom)),
                    _ => Err(serde::de::Error::unknown_variant(
                        value,
                        &[
                            "Ball",
                            "Cuboid",
                            "Capsule",
                            "Segment",
                            "Triangle",
                            "TriMesh",
                            "Polyline",
                            "HalfSpace",
                            "HeightField",
                            "Compound",
                            "ConvexPolyhedron",
                            "Cylinder",
                            "Cone",
                            "RoundCuboid",
                            "RoundTriangle",
                            "RoundCylinder",
                            "RoundCone",
                            "RoundConvexPolyhedron",
                            "Custom",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_str(ShapeTypeSerializableVisitor)
    }
}
