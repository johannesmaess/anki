// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

/* eslint
@typescript-eslint/no-explicit-any: "off",
 */

// A standalone bundle that adds mutateNextCardStates and setupImageCloze
// to the anki namespace. When all clients are using reviewer.js directly, we
// can get rid of this.

import { setupImageCloze } from "../image-occlusion/review";
import { mutateNextCardStates } from "./answering";

globalThis.anki = globalThis.anki || {};
globalThis.anki.mutateNextCardStates = mutateNextCardStates;
globalThis.anki.setupImageCloze = setupImageCloze;
