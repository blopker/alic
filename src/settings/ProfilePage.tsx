import { useNavigate, useParams } from "@solidjs/router";
import { type ImageType, commands } from "../bindings";
import { confirmModal } from "./ConfirmModal";
import {
  SettingBox,
  SettingRow,
  SettingsButton,
  SettingsInput,
  SettingsNumberInput,
  SettingsPage,
  SettingsSelect,
  SettingsToggle,
} from "./SettingsUI";
import { deleteProfile, settings, updateProfile } from "./settingsData";

const imageTypes: ImageType[] = ["JPEG", "PNG", "WEBP", "GIF", "TIFF"];

function ProfilePage() {
  const navigate = useNavigate();
  const params = useParams();
  const data = () => {
    const d = settings.profiles.find(
      (p) => p.id.toString() === params.profileid,
    );
    if (!d) {
      throw new Error(`Profile not found: ${params.profileid}`);
    }
    return d;
  };
  const lossy = () => data().enable_lossy ?? false;
  return (
    <SettingsPage title={`Profile | ${data().name}`}>
      <SettingBox title="Quality">
        <SettingRow
          title="Enable Lossy Compression"
          helpText="Lossy compression reduces the image quality for potentially large space savings. The image may end up looking different."
        >
          <SettingsToggle
            value={lossy()}
            onChange={(value) => {
              updateProfile(data().id, {
                enable_lossy: value,
              });
            }}
          />
        </SettingRow>
        <SettingRow title="JPEG Quality">
          <QualitySlider
            disabled={!lossy()}
            value={data().jpeg_quality}
            onChange={(value) => {
              updateProfile(data().id, { jpeg_quality: value });
            }}
          />
        </SettingRow>
        <SettingRow title="PNG Quality">
          <QualitySlider
            disabled={!lossy()}
            value={data().png_quality}
            onChange={(value) => {
              updateProfile(data().id, { png_quality: value });
            }}
          />
        </SettingRow>
        <SettingRow title="WEBP Quality">
          <QualitySlider
            disabled={!lossy()}
            value={data().webp_quality}
            onChange={(value) => {
              updateProfile(data().id, { webp_quality: value });
            }}
          />
        </SettingRow>
        <SettingRow title="GIF Quality">
          <QualitySlider
            disabled={!lossy()}
            value={data().gif_quality}
            onChange={(value) => {
              updateProfile(data().id, { gif_quality: value });
            }}
          />
        </SettingRow>
      </SettingBox>
      <div class="pt-8" />
      <SettingBox title="Resize">
        <SettingRow
          title="Resize"
          helpText="Resize the image to fit the specified dimensions. Images are not made larger, and are not cropped"
        >
          <SettingsToggle
            value={data().should_resize}
            onChange={(value) => {
              updateProfile(data().id, {
                should_resize: value,
              });
            }}
          />
        </SettingRow>
        <SettingRow title="Resize Width">
          <SettingsNumberInput
            value={data().resize_width}
            onChange={(value) => {
              updateProfile(data().id, {
                resize_width: value,
              });
            }}
          />
          <span class="pl-2">px</span>
        </SettingRow>
        <SettingRow title="Resize Height">
          <SettingsNumberInput
            value={data().resize_height}
            onChange={(value) => {
              updateProfile(data().id, {
                resize_height: value,
              });
            }}
          />
          <span class="pl-2">px</span>
        </SettingRow>
      </SettingBox>
      <div class="pt-8" />
      <SettingBox title="Output">
        <SettingRow
          title="Allow Overwrite"
          helpText="Allow overwriting existing files. Files will only be overwritten if they have the same output name as the input."
        >
          <SettingsToggle
            value={data().should_overwrite}
            onChange={(value) => {
              updateProfile(data().id, {
                should_overwrite: value,
              });
            }}
          />
        </SettingRow>
        <SettingRow
          title="Keep Metadata"
          helpText="Disable to remove all EXIF and color profile information. This may cause colors to change slightly, but removes potentially sensitive or identifying data from the image. For example: location data."
        >
          <SettingsToggle
            value={data().keep_metadata ?? true}
            onChange={(value) => {
              updateProfile(data().id, {
                keep_metadata: value,
              });
            }}
          />
        </SettingRow>
        <SettingRow title="Add Postfix">
          <SettingsToggle
            value={data().add_posfix ?? false}
            onChange={(value) => {
              updateProfile(data().id, {
                add_posfix: value,
              });
            }}
          />
        </SettingRow>
        <SettingRow
          title="Postfix"
          helpText="The postfix to add to the file name. Format: image{postfix}.png"
        >
          <input
            class="w-20 rounded-md border-0 bg-secondary py-1.5 shadow-sm sm:text-sm/6"
            type="text"
            value={data().postfix}
            onInput={(e) => {
              updateProfile(data().id, {
                postfix: e.target.value,
              });
            }}
          />
        </SettingRow>
        <SettingRow
          title="Convert Image"
          helpText="Enable converting files to the specified format."
        >
          <SettingsToggle
            value={data().should_convert}
            onChange={(value) => {
              updateProfile(data().id, {
                should_convert: value,
              });
            }}
          />
        </SettingRow>
        <SettingRow title="Convert Format">
          <SettingsSelect
            class="w-32"
            value={data().convert_extension}
            onChange={(type) =>
              updateProfile(data().id, {
                convert_extension: type as ImageType,
              })
            }
            options={imageTypes}
          />
        </SettingRow>
      </SettingBox>
      <div class="pt-8" />
      <SettingBox title="Manage">
        <SettingRow title="Profile Name">
          <SettingsInput
            label="Name"
            value={data().name}
            onChange={(value) => {
              if (value.length > 1 && value.length < 30) {
                updateProfile(data().id, { name: value });
              }
            }}
          />
        </SettingRow>
        <SettingRow title="Reset">
          <SettingsButton
            onClick={() => {
              confirmModal({
                onConfirm: () => {
                  commands.resetProfile(data().id);
                },
                text: "Are you sure you want to reset this profile?",
              });
            }}
            style="danger"
          >
            Reset
          </SettingsButton>
        </SettingRow>
        <SettingRow title="Delete">
          <SettingsButton
            disabled={data().id === 0}
            onClick={() => {
              confirmModal({
                onConfirm: () => {
                  deleteProfile(data().id);
                  navigate("/settings");
                },
                text: "Are you sure you want to delete this profile?",
              });
            }}
            style="danger"
          >
            Delete
          </SettingsButton>
        </SettingRow>
      </SettingBox>
    </SettingsPage>
  );
}

function QualitySlider(props: {
  value: number;
  onChange: (value: number) => void;
  disabled?: boolean;
}) {
  return (
    <div class="flex gap-4">
      <input
        disabled={props.disabled ?? false}
        type="range"
        min="1"
        max="10"
        value={props.value / 10}
        onInput={(e) => {
          props.onChange(Number.parseInt(e.target.value) * 10);
        }}
      />
      <div>{props.value}</div>
    </div>
  );
}

export { ProfilePage };
