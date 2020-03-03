package com.useoptic.diff.shapes

import com.useoptic.contexts.shapes.Commands.ShapeProvider
import com.useoptic.contexts.shapes.ShapesHelper.{ListKind, NumberKind, ObjectKind}
import com.useoptic.diff.initial.ShapeBuilder
import com.useoptic.diff.interactions.TestHelpers
import org.scalatest.FunSpec
import com.useoptic.diff.shapes.JsonTrailPathComponent._
import io.circe.literal._


class ResolverSpec extends FunSpec {
  describe("resolving trails") {
    describe("given a spec with a request body that is an object") {
      val builtShape = new ShapeBuilder(json"""{"f":[123]}""", "s").run
      val rfcState = TestHelpers.fromCommands(builtShape.commands)


      it("should resolve the root as an object") {
        val resolvedTrail = Resolvers.resolveTrailToCoreShape(rfcState, ShapeTrail(builtShape.rootShapeId, Seq()))
        assert(resolvedTrail == ResolvedTrail(rfcState.shapesState.shapes("s_0"), ObjectKind, Map()))
      }

      it("should resolve the field 'f'") {
        val resolvedTrail = Resolvers.resolveTrailToCoreShape(rfcState, ShapeTrail(builtShape.rootShapeId, Seq(ObjectFieldTrail("s_1", "s_2"))))
        assert(resolvedTrail == ResolvedTrail(rfcState.shapesState.shapes("s_2"), ListKind, Map(ListKind.innerParam -> Some(ShapeProvider("s_3")))))
      }

      it("should resolve a value in field 'f'") {
        val resolvedTrail = Resolvers.resolveTrailToCoreShape(rfcState, ShapeTrail(builtShape.rootShapeId, Seq(ObjectFieldTrail("s_1", "s_2"), ListItemTrail("s_2", "s_3"))))
        assert(resolvedTrail == ResolvedTrail(rfcState.shapesState.shapes("s_3"), NumberKind, Map(ListKind.innerParam -> Some(ShapeProvider("s_3")))))
      }
    }
    describe("given a spec with a request body that is an array") {

      val builtShape = new ShapeBuilder(json"""[{"id": 1}]""", "s").run
      val rfcState = TestHelpers.fromCommands(builtShape.commands)
      it("should resolve the root as a list") {
        val resolvedTrail = Resolvers.resolveTrailToCoreShape(rfcState, ShapeTrail(builtShape.rootShapeId, Seq()))
        assert(resolvedTrail == ResolvedTrail(rfcState.shapesState.shapes("s_0"), ListKind, Map()))
      }
      it("should resolve the list item as an object") {
        val resolvedTrail = Resolvers.resolveTrailToCoreShape(rfcState, ShapeTrail(builtShape.rootShapeId, Seq(ListItemTrail("s_0", "s_1"))))
        assert(resolvedTrail == ResolvedTrail(rfcState.shapesState.shapes("s_1"), ObjectKind, Map()))
      }
      it("should resolve the field id as a number") {
        val resolvedTrail = Resolvers.resolveTrailToCoreShape(rfcState, ShapeTrail(builtShape.rootShapeId, Seq(ListItemTrail("s_0", "s_1"), ObjectFieldTrail("s_2", "s_3"))))
        assert(resolvedTrail == ResolvedTrail(rfcState.shapesState.shapes("s_3"), NumberKind, Map()))
      }
    }
  }


  describe("resolving json from trails and interactions") {
    describe("primitives") {
      it("should resolve the value") {

        val resolved = Resolvers.tryResolveJsonTrail(JsonTrail(Seq()), Some(json""""string""""))
        assert(resolved.contains(json""""string""""))
      }
    }
    describe("object") {
      it("should resolve the value") {
        val resolved = Resolvers.tryResolveJsonTrail(JsonTrail(Seq(JsonObject(), JsonObjectKey("k"))), Some(json"""{"k":1}"""))
        assert(resolved.contains(json"""1"""))
      }
      it("should not resolve a value") {
        val resolved = Resolvers.tryResolveJsonTrail(JsonTrail(Seq(JsonObject(), JsonObjectKey("notk"))), Some(json"""{"k":1}"""))
        assert(resolved.isEmpty)
      }
    }
    describe("array") {
      it("should resolve the value") {
        val resolved = Resolvers.tryResolveJsonTrail(JsonTrail(Seq(JsonArray(), JsonArrayItem(0))), Some(json"""[1]"""))
        assert(resolved.contains(json"""1"""))
      }
      it("should not resolve a value") {
        val resolved = Resolvers.tryResolveJsonTrail(JsonTrail(Seq(JsonArray(), JsonArrayItem(1))), Some(json"""[1]"""))
        assert(resolved.isEmpty)
      }
    }
    describe("nested") {
      val nested = json"""{"k":[{"x": ["a", "b"]}]}"""
      it("should resolve .k[0].x[1] => 'b'") {
        val trail = JsonTrail(Seq(JsonObject(), JsonObjectKey("k"), JsonArray(), JsonArrayItem(0), JsonObject(), JsonObjectKey("x"), JsonArray(), JsonArrayItem(1)))
        val resolved = Resolvers.tryResolveJsonTrail(trail, Some(nested))
        assert(resolved.contains(json""""b""""))
      }
      it("should not resolve .k[0].x[2]") {
        val trail = JsonTrail(Seq(JsonObject(), JsonObjectKey("k"), JsonArray(), JsonArrayItem(0), JsonObject(), JsonObjectKey("x"), JsonArray(), JsonArrayItem(2)))
        val resolved = Resolvers.tryResolveJsonTrail(trail, Some(nested))
        assert(resolved.isEmpty)
      }
    }
  }
}